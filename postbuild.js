const path = require('node:path');
const fs = require('node:fs');

console.log("## Starting Post-build process ##");

const packagePath = path.resolve('package');

const dirs = fs.readdirSync(packagePath, { withFileTypes: true })
    .filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name);

dirs.forEach(dir => {
   const source = path.resolve(packagePath, dir);

   fs.readdirSync(source).forEach(file => {
       const filePath = path.resolve(source, file);

       if (file.includes('mina_signer_wasm') && !file.includes('.wasm')) {
           const extension = file.split('.').slice(1).join('.');

           fs.renameSync(filePath, path.resolve(source, `index.${extension}`));
       }

       if (['LICENSE', 'package.json', 'README.md', '.gitignore'].some(name => file.includes(name)))
           fs.unlinkSync(filePath);
   });
});

console.log("## Post-build successfully finished ##");
