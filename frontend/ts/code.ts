import hljs from "highlight.js";
import { languageExtensions } from "./languages_extension";

const codeArea = document.getElementById("codeArea")! as HTMLTextAreaElement;

hljs.highlightElement(codeArea);

const detectedLanguage =
  codeArea.className
    .split(" ")
    .find((cls) => cls.startsWith("language-"))
    ?.substring(9) ?? "txt";

setTimeout(
  () => document.getElementById("codeAreaPre")!.classList.remove("hidden"),
  100,
);

document.getElementById("generateFile")!.onclick = () => {
  if (codeArea.textContent === null) return;

  let data = new Blob([codeArea.textContent], {
    type: "text/plain",
  });
  const objectURL = window.URL.createObjectURL(data);
  let a = document.createElement("a");
  a.style.display = "none";
  a.href = objectURL;
  a.download = `${window.location.pathname.split("/")[1]}.${languageExtensions[detectedLanguage] ?? "txt"}`;
  a.click();
  window.URL.revokeObjectURL(objectURL);
};
