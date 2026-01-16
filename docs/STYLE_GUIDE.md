# Oisko-t-it Style Guide

## Core Colors
| Name | Hex | Usage |
|------|-----|-------|
| **Background** | `#0f1116` | Main body background |
| **Card / Surface** | `#1e293b` (or `white/5`) | Cards, panels (often with blur) |
| **Text Primary** | `#ffffff` | Headings, primary text |
| **Text Secondary**| `#9ca3af` (Gray-400)| Subtitles, descriptions |
| **Accent** | `#91a4d2` | Links, buttons, active states |

## Typography
- **Font Family**: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif
- **Headings**: Bold, White
- **Body**: Regular, legible contrast

## UV Patterns
### Glassmorphism
- Background: `rgba(255, 255, 255, 0.05)`
- Border: `1px solid rgba(255, 255, 255, 0.1)`
- Blur: `backdrop-filter: blur(10px)`

### Buttons
- **Primary**: Solid Accent `#91a4d2` (or Tailwind `indigo-500` variant), White Text, Hover darken/glow.
- **Ghost**: Transparent background, Accent Border/Text.

## Component Specifics
### Login Page
- **Layout**: Centered Card
- **Inputs**: Dark background (`gray-900` or transparent), White Text, Subtle Gray Border (`gray-700` or `white/10`).
- **Feedback**: Red for errors, Green for success.
