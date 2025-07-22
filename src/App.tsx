import { useState, useCallback } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { readFile, writeFile } from "@tauri-apps/plugin-fs";
import "./App.css";

function App() {
  const [originalImage, setOriginalImage] = useState<string | null>(null);
  const [upscaledImage, setUpscaledImage] = useState<string | null>(null);
  const [originalImagePath, setOriginalImagePath] = useState<string | null>(
    null
  );
  const [upscaledImagePath, setUpscaledImagePath] = useState<string | null>(
    null
  );
  const [upscaleFactor, setUpscaleFactor] = useState(2);
  const [isUpscaling, setIsUpscaling] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [moltenVkStatus, setMoltenVkStatus] = useState<string | null>(null);

  const handleSelectImage = useCallback(async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "Image",
            extensions: ["png", "jpeg", "jpg", "webp", "bmp"],
          },
        ],
      });

      if (selected && typeof selected === "string") {
        setOriginalImagePath(selected);
        setOriginalImage(convertFileSrc(selected));
        setUpscaledImage(null);
        setError(null);
      }
    } catch (err) {
      console.error(err);
      setError("Failed to open image.");
    }
  }, []);

  const handleUpscale = async () => {
    if (!originalImagePath) return;

    setIsUpscaling(true);
    setError(null);
    setUpscaledImage(null);
    setUpscaledImagePath(null);

    try {
      // This is where you would call your Rust backend
      const upscaledPath = await invoke<string>("upscale_image", {
        path: originalImagePath,
        factor: upscaleFactor,
      });

      setUpscaledImagePath(upscaledPath);
      setUpscaledImage(convertFileSrc(upscaledPath));
    } catch (err) {
      console.error(err);
      setError(
        "Failed to upscale image. Make sure the backend is running and the command is correct."
      );
    } finally {
      setIsUpscaling(false);
    }
  };

  const handleDownload = async () => {
    if (!upscaledImagePath || !originalImagePath) return;

    try {
      const originalFilename = originalImagePath.substring(
        originalImagePath.lastIndexOf("/") + 1,
        originalImagePath.lastIndexOf(".")
      );
      const suggestedFilename = `upscaled-${originalFilename}.png`;

      const filePath = await save({
        title: "Save Upscaled Image",
        defaultPath: suggestedFilename,
        filters: [
          {
            name: "PNG Image",
            extensions: ["png"],
          },
        ],
      });

      if (filePath) {
        const contents = await readFile(upscaledImagePath);
        await writeFile(filePath, contents);
      }
    } catch (err) {
      console.error(err);
      setError("Failed to save image.");
    }
  };

  const clearImages = () => {
    setOriginalImage(null);
    setOriginalImagePath(null);
    setUpscaledImage(null);
    setUpscaledImagePath(null);
    setError(null);
  };

  const testMoltenVk = async () => {
    try {
      setMoltenVkStatus("Testing MoltenVK...");
      const result = await invoke<string>("test_moltenvk_setup");
      setMoltenVkStatus(`✅ ${result}`);
    } catch (err) {
      console.error(err);
      setMoltenVkStatus(`❌ MoltenVK test failed: ${err}`);
    }
  };

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>V-Upscale</h1>
        <p>Cross-platform AI Image Upscaler powered by Vulkan</p>
        <div className="moltenvk-test">
          <button onClick={testMoltenVk} className="test-button">
            Test MoltenVK
          </button>
          {moltenVkStatus && <p className="status-message">{moltenVkStatus}</p>}
        </div>
      </header>

      <main className="app-main">
        {!originalImage ? (
          <div className="upload-area" onClick={handleSelectImage}>
            <p>Click or drag & drop to upload an image</p>
            <span>Supports PNG, JPG, WEBP, BMP</span>
          </div>
        ) : (
          <div className="preview-area">
            <div className="image-container">
              <h2>Original</h2>
              <img src={originalImage} alt="Original" />
            </div>
            <div
              className={`image-container ${isUpscaling ? "processing" : ""}`}
            >
              <h2>Upscaled</h2>
              {isUpscaling && (
                <div className="loader-container">
                  <div className="loader"></div>
                  <span>Upscaling, please wait...</span>
                </div>
              )}
              {error && <p className="error-message">{error}</p>}
              {upscaledImage && !isUpscaling && (
                <img src={upscaledImage} alt="Upscaled" />
              )}
              {!upscaledImage && !isUpscaling && (
                <div className="placeholder">Result will appear here</div>
              )}
            </div>
          </div>
        )}
      </main>

      {originalImage && (
        <footer className="app-controls">
          <div className="upscale-options">
            <span>Upscale Factor:</span>
            <label>
              <input
                type="radio"
                name="factor"
                value={2}
                checked={upscaleFactor === 2}
                onChange={() => setUpscaleFactor(2)}
              />{" "}
              2x
            </label>
            <label>
              <input
                type="radio"
                name="factor"
                value={4}
                checked={upscaleFactor === 4}
                onChange={() => setUpscaleFactor(4)}
              />{" "}
              4x
            </label>
            <label>
              <input
                type="radio"
                name="factor"
                value={8}
                checked={upscaleFactor === 8}
                onChange={() => setUpscaleFactor(8)}
              />{" "}
              8x
            </label>
          </div>
          <div className="action-buttons">
            <button onClick={handleUpscale} disabled={isUpscaling}>
              {isUpscaling ? "Upscaling..." : "Upscale Image"}
            </button>
            <button
              onClick={handleDownload}
              disabled={!upscaledImagePath || isUpscaling}
            >
              Download
            </button>
            <button
              onClick={clearImages}
              disabled={isUpscaling}
              className="secondary"
            >
              Clear
            </button>
          </div>
        </footer>
      )}
    </div>
  );
}

export default App;
