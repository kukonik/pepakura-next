const fs = require('fs');
const path = require('path');

const distDir = path.join(__dirname, 'dist');
const indexHtmlPath = path.join(distDir, 'index.html');

if (!fs.existsSync(indexHtmlPath)) {
  console.error('index.html not found in dist/');
  process.exit(1);
}

let content = fs.readFileSync(indexHtmlPath, 'utf-8');

// Исправляем абсолютные пути на относительные
content = content
  .replace(/src="\/assets\//g, 'src="./assets/')
  .replace(/href="\/assets\//g, 'href="./assets/');

fs.writeFileSync(indexHtmlPath, content, 'utf-8');
console.log('Fixed asset paths in index.html');
