import hljs from "highlight.js";

const codeArea = document.getElementById("codeArea")! as HTMLTextAreaElement;

document.getElementById("saveButton")!.onclick = async () => {
  const content = codeArea.value.trim();
  if (!content) return;

  const { language: detectedLanguage } = hljs.highlightAuto(content);

  const response = await fetch("/new", {
    method: "POST",
    headers: {
      "Content-Type": "text/plain",
      Language: detectedLanguage || "txt",
    },
    mode: "cors",
    cache: "default",
    body: content,
    redirect: "follow",
  });
  const responseContent = await response.json();
  if (responseContent?.status === "error") {
    // TODO
    return;
  }

  history.pushState({}, "", response.headers.get("Location"));
  window.location.reload();
};

codeArea.focus();
codeArea.onkeydown = (e) => {
  if (e.key !== "Tab") return;

  e.preventDefault();

  const start = codeArea.selectionStart;
  const end = codeArea.selectionEnd;

  codeArea.value =
    codeArea.value.substring(0, start) + "\t" + codeArea.value.substring(end);
  codeArea.selectionStart = codeArea.selectionEnd = start + 1;
};
