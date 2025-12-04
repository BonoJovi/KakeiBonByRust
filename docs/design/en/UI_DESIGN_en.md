# UI Design Document

**Last Updated**: 2025-12-05 04:36 JST

## Table of Contents
1. [Overview](#overview)
2. [Design Principles](#design-principles)
3. [Common UI Elements](#common-ui-elements)
4. [Screen List](#screen-list)
5. [Responsive Design](#responsive-design)
6. [Accessibility](#accessibility)

---

## Overview

This document describes the user interface design of KakeiBon.

### Technology Stack
- **HTML5**: Semantic markup
- **CSS3**: Modern layout (Flexbox/Grid)
- **Vanilla JavaScript**: Framework-free, ES6+ modules

### Design Philosophy
- **Simplicity**: Avoid complex frameworks to reduce learning costs
- **Maintainability**: Modular common components
- **Consistency**: Unified UI/UX across all screens

---

## Design Principles

### 1. Consistency
- **Colors**: Unified color palette (primary, secondary, error, success)
- **Typography**: Consistent font sizes and styles
- **Spacing**: Consistent margins and padding

### 2. Intuitiveness
- **Clear Labels**: Descriptive labels for all input fields
- **Immediate Feedback**: Instant validation error display
- **Visual Hierarchy**: Emphasis on important elements

### 3. Accessibility
- **Keyboard Navigation**: All operations executable via keyboard
- **Screen Reader Support**: Proper ARIA attributes (future implementation)
- **Color Contrast**: WCAG 2.1 AA compliance

### 4. Performance
- **Lightweight**: No external frameworks
- **Lazy Loading**: Load only necessary modules
- **Minification**: Optimized CSS and JavaScript

---

## Common UI Elements

### Menu Bar
**Module**: `res/js/modules/menu-bar.js`

**Features**:
- User info display
- Language switching (Japanese/English)
- Font size adjustment
- Logout functionality

**Usage**:
```javascript
import { loadMenuBar } from './modules/menu-bar.js';
```

### Low Vision Support Indicator
**Stylesheet**: `res/css/indicators.css`

**Purpose**: Enhance visual feedback for users with low vision, clearly communicating UI element states

**Types**:
- **Active/Selected State**: High-contrast borders and highlights
- **Focus**: Thick borders to clearly indicate current focus position
- **Disabled State**: Grayscale and patterns for disabled elements
- **Status Feedback**: High-contrast display for success (green), error (red), info (blue)

**Future Implementation**: Screen reader support (ARIA attributes)

### Font Size Adjustment
**Module**: `res/js/modules/font-size.js`

**Size Options**:
- Small: 12px
- Medium: 16px (default)
- Large: 20px
- Custom: User-defined (12-32px)

**Persistence**: localStorage

### Modal Dialogs
**Common Pattern**:
- Semi-transparent overlay
- Centered content
- Close button (×)
- Close with Esc key

---

## Screen List

### 1. Login/Admin Setup Screen
**File**: `res/index.html`

**Features**:
- Admin setup (first launch)
- User login
- Language selection
- Font size adjustment

### 2. User Management Screen
**File**: `res/user-management.html`

**Features**:
- User list display
- Add/Edit/Delete users
- Password change
- Role management (Admin/User)

### 3. Category Management Screen
**File**: `res/category.html`

**Features**:
- 3-level hierarchy (CATEGORY1/2/3)
- Add/Edit/Delete categories
- Parent-child relationship management

### 4. Account Management Screen
**File**: `res/account.html`

**Features**:
- Bank account management
- Add/Edit/Delete accounts
- Balance display

### 5. Shop Management Screen
**File**: `res/shop.html`

**Features**:
- Shop information management
- Enable/Disable function (IS_DISABLED)
- Shop search

### 6. Manufacturer Management Screen
**File**: `res/manufacturer.html`

**Features**:
- Manufacturer information management
- Enable/Disable function (IS_DISABLED)
- Manufacturer search

### 7. Product Management Screen
**File**: `res/product.html`

**Features**:
- Product information management
- Manufacturer linkage
- Enable/Disable function (IS_DISABLED)
- Product search

### 8. Transaction Management Screen
**File**: `res/transaction.html`

**Features**:
- Transaction header management
- Transaction detail management
- Smart tax calculation
- Rounding error auto-detection

---

## Responsive Design

### Breakpoints
- **Mobile**: < 768px
- **Tablet**: 768px - 1024px
- **Desktop**: > 1024px

### Layout Strategy
- **Mobile-First**: Design for mobile and scale up
- **Flexible Grid**: CSS Grid and Flexbox
- **Adaptive Components**: Components adapt to screen size

---

## Accessibility

### Keyboard Support
- **Tab Navigation**: Move between elements
- **Enter**: Submit/Confirm
- **Esc**: Close modal dialogs

### Visual Support
- **High Contrast**: Clear foreground/background contrast
- **Focus Indicators**: Visible focus state
- **Error Indicators**: Clear error messages

### Future Implementation
- **Screen Reader Support**: Complete ARIA attributes
- **Voice Control**: Voice command support
- **High Contrast Theme**: Enhanced high contrast mode

---

**Related Documents**:
- [Architecture Design](ARCHITECTURE_en.md)
- [Database Design](DATABASE_DESIGN_en.md)
- [Security Design](SECURITY_DESIGN_en.md)
- [API Common](../api/en/API_COMMON_en.md)

---

**Last Updated**: 2025-12-05 04:36 JST
