import hljs from "highlight.js";
import { languageExtensions } from "./languages_extension";

const codeContainer = document.getElementById("codeContainer")!;
const { value: highlightedCode, language: detectedLanguage } =
  hljs.highlightAuto(codeContainer.textContent || "");
codeContainer.innerHTML = highlightedCode;
codeContainer.classList.add("hljs");
codeContainer.classList.remove("hidden");

document.getElementById("generateFile")!.onclick = () => {
  if (codeContainer.innerText === null) return;

  const data = new Blob([codeContainer.innerText], {
    type: "text/plain",
  });
  const objectURL = window.URL.createObjectURL(data);
  const downloadLink = document.createElement("a");
  downloadLink.style.display = "none";
  downloadLink.href = objectURL;
  downloadLink.download = `${window.location.pathname.split("/")[1]}.${languageExtensions[detectedLanguage ?? ""] ?? "txt"}`;
  downloadLink.click();
  window.URL.revokeObjectURL(objectURL);
};
