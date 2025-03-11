import path from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Create require() for ES modules
const require = createRequire(import.meta.url);

const binaryPathMap: { [key: string]: string } = {
  darwin: "index-darwin-arm64.node",
  win32: "index-win32.node",
};

const binaryPath = binaryPathMap[process.platform];

if (!binaryPath) {
  throw new Error(`Unsupported platform: ${process.platform}`);
}

const bindingPath = path.join(__dirname, "../bindings", binaryPath);

// Load the native `.node` file
const { convertToAvif } = require(bindingPath);

/**
 * Convert an image to AVIF format using Neon bindings.
 * @param {string} inputPath - Path to the input image file.
 * @param {number} [quality=50] - AVIF quality (0-100).
 * @param {number} [speed=4] - Encoding speed (0-10).
 * @returns {Promise<Buffer>} A Promise resolving to a Buffer containing the AVIF image.
 */
export function convertToAvifSync(
  inputPath: string,
  quality: number = 50,
  speed: number = 4
): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    try {
      const avifBuffer = convertToAvif(inputPath, quality, speed);
      resolve(avifBuffer);
    } catch (error) {
      reject(error);
    }
  });
}
