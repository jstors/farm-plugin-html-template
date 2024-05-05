# farm-plugin-html-template
<a href="https://www.npmjs.com/package/@jstors/farm-plugin-html-template">![NPM Version](https://img.shields.io/npm/v/%40jstors%2Ffarm-plugin-html-template)</a>



This plugin is used to dynamically replace variables like `${var}$` injected into HTML.

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

Note that if you want to replace the variables correctly, you must define them correctly inside the configured `data`.

If there is no match in the `data` field for the variable you defined, then it won't be replaced.

```javascript
import farmPluginHtmlTemplate from '@jstors/arm-plugin-html-template';
import { defineConfig } from "@farmfe/core";

export default defineConfig({
  // plugin configuration
  plugins: [
    ["@jstors/farm-plugin-html-template",
     {
      template: path.resolve(__dirname, 'index.html'),
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

- `pre-conversion`
```html
<!-- .... -->
<title>${title}$</title>
<meta name="description" content="${description}$">
<link rel="stylesheet" href="${css_link}$">
<!-- ... -->
```
- `converted`

```html
<!-- .... -->
<title>Hello World</title>
<meta name="description" content="${description}$">
<link rel="stylesheet" href="${css_link}$">
<!-- ... -->
```

## Options

- `template`: The path to the HTML file that will be used as a template.
- `data`: An object containing the variables that will be injected into the HTML file.
