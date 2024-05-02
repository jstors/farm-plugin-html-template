# farm-plugin-html-template

This plugin is used to dynamically replace variables injected into HTML.

## Installation

```bash
npm install @jstors/farm-plugin-html-template
# or
yarn add @jstors/farm-plugin-html-template
#or 
pnpm install @jstors/farm-plugin-html-template
```
## Usage

### Configuration
```javascript
import farmPluginHtmlTemplate from '@jstors/arm-plugin-html-template';
import { defineConfig } from "@farmfe/core";

export default defineConfig({
  // plugin configuration
  plugins: [
    ["@jstors/farm-plugin-html-template",
     {
      template: 'src/index.html',
      data: {
        title: 'Hello World',
        description: 'This is a description',
        keywords: 'html, template, farm'
      }
    }]
  ],
});
```
### HTML Template
```html
<!-- .... -->
<title>${title}$</title>
<meta name="description" content="${description}$">
<link rel="stylesheet" href="${css_link}$">
<!-- ... -->
```

## Options

- `template`: The path to the HTML file that will be used as a template.
- `data`: An object containing the variables that will be injected into the HTML file.
