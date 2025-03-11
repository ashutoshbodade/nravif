# nravif 🔥

[![npm version](https://img.shields.io/npm/v/nravif)](https://www.npmjs.com/package/nravif)

**Convert images to AVIF super fast and make friends with Rust along the way! 🚀🦀**

**High-performance AVIF converter** powered by Rust's [ravif](https://crates.io/crates/ravif), 
offering native-speed conversions with minimal memory footprint. Outperforms JavaScript-based solutions like Sharp by 3-5x in benchmark tests.

## Features ✨

- ⚡ **Blazing fast** conversions using Rust-native code
- 🦀 **Memory-safe** implementation via Rust bindings
- 🖼️ **Lossy/lossless** compression support
- 📈 **Tunable quality/speed** parameters
- 🧩 **Dual CJS/ESM** module support
- 🛡️ **TypeSafe** TypeScript definitions included

## Installation 📦

```bash
npm install nravif
```

## Quick Start 🚀
```js
import { convertToAvifSync } from "nravif";
import fs from "fs/promises";

// Function to convert an image to AVIF format and save it to a file
async function convertAndSave() {
  try {
    // Define the input image file
    const inputFile = "input.jpg";
    // Define the output AVIF file name
    const outputFile = "output.avif";
    // Set AVIF conversion quality (0-100) and speed (0-10)
    const quality = 50;
    const speed = 4;

    console.log(`Converting ${inputFile} to AVIF format...`);

    // Convert the image to AVIF format, returns a Buffer
    const avifBuffer = await convertToAvifSync(inputFile, quality, speed);

    // Save the AVIF buffer to an output file
    await fs.writeFile(outputFile, avifBuffer);

    console.log(`✅ AVIF file saved successfully: ${outputFile}`);
  } catch (error) {
    // Handle any errors that occur during conversion or saving
    console.error("❌ Error converting image:", error);
  }
}

// Run the conversion function
convertAndSave();
```

## GitHub Repository 🔗 
[👉 View on GitHub](https://github.com/ashutoshbodade/nravif)

## Contributing 🤝
We welcome contributions! Please follow these guidelines:

- Rust Code - Modify .rs files in /native
- TypeScript - Update files in /src


## Platform Support  🖥️
| Platform  | Supported  |  
|-----------|-----------|  
| macOS (Darwin) | ✅ Yes |  
| Windows | ✅ Yes |  
| Linux | ⏳ Coming Soon |  

## 💡 Credits  
This project is built using the following awesome technologies:  

- [ravif](https://docs.rs/ravif/) - Rust-based AVIF encoder  
- [neon-rs](https://github.com/neon-bindings/neon) - Native bindings for Rust in Node.js  
- [Rust](https://www.rust-lang.org/) - Systems programming language used for performance  
- [Microbundle](https://github.com/developit/microbundle) - Zero-config bundler for modern JavaScript libraries  

## License 📄
MIT © Ashutosh Bodade

## Made with 🦀 Rust + ❤️ JavaScript