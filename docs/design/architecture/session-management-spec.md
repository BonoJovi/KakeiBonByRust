# Session Management Specification

## Purpose
- Manage user session information during application runtime
- Ensure secure handling of authentication state without disk persistence

## Session Information Storage

### Storage Location
- **Tauri State** (memory only, no persistence)

### Data to Store
Minimum required values:
- `user_id`: User identifier
- `role`: User role (admin/user)
- **`source_screen`**: Source screen name (translation resource key prefix before the dot)
  - Example: `shop_mgmt` (from `shop_mgmt.title`)
  - Purpose: Track navigation source for proper back navigation and context preservation
- **`category1_code`**: Category 1 code for transaction context
  - Example: `INCOME`, `EXPENSE`
  - Purpose: Required when transitioning from transaction registration/edit screen to detail management screen
  - Used to maintain category context across screen transitions

## Session Lifecycle

### Session Validity
- Valid only between login and logout
- `user_id`, `role`, `source_screen`, and `category1_code` are NOT persisted
- Application restart always redirects to login screen
- This policy remains unchanged even in release builds

### Logout Processing
- Clear (destroy) `user_id`, `role`, `source_screen`, and `category1_code` held in Tauri State

### Screen Transition Validation
- Use `user_id` and `role` from Tauri State as keys
- Query USERS table to confirm user existence
- Validates session integrity on each screen transition

## Security Considerations

### Data Protection
- `user_id`, `role`, `source_screen`, and `category1_code` are held in memory only (no persistence)
- Tauri State is type-safe and inaccessible directly from frontend
- Encryption is unnecessary (no disk writes)
- Session information retrieval must be through Tauri Commands only

### Security Benefits
- Data saved in memory only, not written to disk
- Automatically cleared when application exits
- Frontend cannot access directly (only via Tauri Commands)
- No encryption needed (no persistence)

## Implementation Architecture

### Rust Backend Structure

```rust
use tauri::{Manager, State};
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    role: i32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct SessionInfo {
    source_screen: Option<String>,
    category1_code: Option<String>,
}

#[derive(Default)]
struct AuthState {
    current_user: Mutex<Option<User>>,
    session_info: Mutex<SessionInfo>,
}

#[tauri::command]
async fn login(
    username: String,
    password: String,
    state: State<'_, AuthState>,
    app: tauri::AppHandle,
) -> Result<User, String> {
    // Authentication process (DB verification, etc.)
    let user = authenticate(username, password).await?;

    // Update in-memory state (no persistence)
    *state.current_user.lock().unwrap() = Some(user.clone());

    // Notify all windows
    app.emit_all("auth-changed", &user)?;

    Ok(user)
}

#[tauri::command]
async fn logout(
    state: State<'_, AuthState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Clear in-memory state
    *state.current_user.lock().unwrap() = None;
    *state.session_info.lock().unwrap() = SessionInfo::default();

    // Notify all windows
    app.emit_all("auth-changed", ())?;

    Ok(())
}

#[tauri::command]
fn get_current_user(state: State<AuthState>) -> Option<User> {
    state.current_user.lock().unwrap().clone()
}

#[tauri::command]
fn is_authenticated(state: State<AuthState>) -> bool {
    state.current_user.lock().unwrap().is_some()
}

#[tauri::command]
fn set_source_screen(
    source_screen: String,
    state: State<'_, AuthState>,
) -> Result<(), String> {
    state.session_info.lock().unwrap().source_screen = Some(source_screen);
    Ok(())
}

#[tauri::command]
fn get_source_screen(state: State<'_, AuthState>) -> Option<String> {
    state.session_info.lock().unwrap().source_screen.clone()
}

#[tauri::command]
fn set_category1_code(
    category1_code: String,
    state: State<'_, AuthState>,
) -> Result<(), String> {
    state.session_info.lock().unwrap().category1_code = Some(category1_code);
    Ok(())
}

#[tauri::command]
fn get_category1_code(state: State<'_, AuthState>) -> Option<String> {
    state.session_info.lock().unwrap().category1_code.clone()
}

fn main() {
    tauri::Builder::default()
        .manage(AuthState::default())
        // No setup() needed - always start in unauthenticated state
        .invoke_handler(tauri::generate_handler![
            login,
            logout,
            get_current_user,
            is_authenticated,
            set_source_screen,
            get_source_screen,
            set_category1_code,
            get_category1_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Frontend Implementation

```javascript
// auth.js
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface User {
  id: string;
  username: string;
  role: number;
}

// Login
export async function login(username: string, password: string): Promise<User> {
  return await invoke('login', { username, password });
}

// Logout
export async function logout(): Promise<void> {
  await invoke('logout');
}

// Get current user
export async function getCurrentUser(): Promise<User | null> {
  return await invoke('get_current_user');
}

// Check authentication status
export async function isAuthenticated(): Promise<boolean> {
  return await invoke('is_authenticated');
}

// Set source screen
export async function setSourceScreen(sourceScreen: string): Promise<void> {
  await invoke('set_source_screen', { sourceScreen });
}

// Get source screen
export async function getSourceScreen(): Promise<string | null> {
  return await invoke('get_source_screen');
}

// Set category1 code
export async function setCategory1Code(category1Code: string): Promise<void> {
  await invoke('set_category1_code', { category1Code });
}

// Get category1 code
export async function getCategory1Code(): Promise<string | null> {
  return await invoke('get_category1_code');
}

// Monitor authentication state changes (shared across all windows)
export function onAuthChanged(callback: (user: User | null) => void) {
  return listen('auth-changed', (event) => {
    callback(event.payload as User | null);
  });
}
```

### React/Svelte Usage Example

```javascript
// React example
import { useEffect, useState } from 'react';
import { getCurrentUser, onAuthChanged, type User } from './lib/auth';

function App() {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    // Get initial state
    getCurrentUser().then(setUser);

    // Monitor authentication state changes (reflects login/logout in other windows)
    const unlisten = onAuthChanged((newUser) => {
      setUser(newUser);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  if (!user) {
    return <LoginPage />;
  }

  return <Dashboard user={user} />;
}
```

## Multi-Window Architecture

```
┌─────────────────────────────────────────┐
│  Main Window                            │
│  ┌──────────────────────────────────┐   │
│  │ Login Form                       │   │
│  └──────────────────────────────────┘   │
└─────────────────────────────────────────┘
           ↓ Login successful
┌─────────────────────────────────────────┐
│  Rust Backend (Tauri)                   │
│  ┌──────────────────────────────────┐   │
│  │ Tauri State (Memory Only)       │   │
│  │ - user_id, role                 │   │
│  │ - source_screen                 │   │
│  │ - category1_code                │   │
│  │ - Automatically cleared on exit │   │
│  └──────────────────────────────────┘   │
└─────────────────────────────────────────┘
           ↓ State synchronization
┌────────────┐  ┌────────────┐  ┌────────────┐
│ Window 1   │  │ Window 2   │  │ Window 3   │
│ (Authed)   │  │ (Authed)   │  │ (Authed)   │
└────────────┘  └────────────┘  └────────────┘
```

## Why Not localStorage/sessionStorage?

| Requirement | localStorage | sessionStorage | Tauri State (Memory) |
|------------|--------------|----------------|----------------------|
| Multi-window sharing | ❌ Difficult | ❌ Impossible | ✅ Possible |
| Security | ❌ XSS vulnerable | ❌ XSS vulnerable | ✅ Safe |
| Persistence | ✅ Possible | ❌ Impossible | ❌ By design |
| Encryption | ❌ Impossible | ❌ Impossible | ✅ Not needed (memory only) |
| Real-time sync | ❌ Difficult | ❌ Impossible | ✅ Via events |

## Implementation Flow

1. Implement AuthState and Tauri Commands on Rust side
2. Create auth.js on frontend side
3. Set up onAuthChanged listeners on each screen
4. Always start in unauthenticated state on app launch

## Common Pitfalls

- Don't forget Mutex locking
- Always clean up event listeners
- No persistence plugin needed (use Tauri State only)

## Design Policy Confirmation

- ✅ Manage in memory only (no persistence)
- ✅ Always redirect to login screen on app restart
- ✅ This policy remains unchanged in release builds

---

**Last Updated**: 2025-11-13 22:51 JST
