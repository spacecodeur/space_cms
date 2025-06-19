# SCSS Architecture

This directory contains the SCSS styles for the Space CMS web application, organized using a modular architecture that supports responsive design across mobile, tablet, and desktop devices.

## Directory Structure

```
style/
├── abstracts/      # Variables, mixins, functions, breakpoints
├── base/          # Reset, typography, base styles
├── components/    # Component-specific styles
├── layout/        # Layout components (header, footer, etc.)
├── pages/         # Page-specific styles
└── main.scss      # Main entry point
```

## Breakpoints

We use the same breakpoints as Tailwind CSS for consistency:

- `sm`: 640px (40rem)
- `md`: 768px (48rem) 
- `lg`: 1024px (64rem)
- `xl`: 1280px (80rem)
- `2xl`: 1536px (96rem)

## Usage Examples

### Using Breakpoint Mixins

```scss
.component {
    // Mobile-first approach
    padding: 1rem;
    
    @include media-up(md) {
        padding: 2rem;
    }
    
    @include media-up(lg) {
        padding: 3rem;
    }
}
```

### Convenience Mixins

```scss
.mobile-only-element {
    @include mobile-only {
        display: block;
    }
}

.desktop-navigation {
    @include desktop-up {
        display: flex;
    }
}
```

## Adding New Styles

1. **Components**: Create a new file in `components/` with underscore prefix (e.g., `_button.scss`)
2. **Import**: Add the import to the relevant `_index.scss` file
3. **Variables**: Use variables from `abstracts/_variables.scss` for consistency
4. **Responsive**: Use breakpoint mixins for responsive design

## Best Practices

- Write mobile-first styles
- Use variables for colors, spacing, and typography
- Leverage mixins for common patterns
- Keep component styles isolated and reusable
- Follow BEM naming convention for class names