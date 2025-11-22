import { languageExtensions } from "./languages_extension";

document.getElementById("generateFile")!.onclick = async () => {
  const rawUrl = document.getElementById("rawUrl")! as HTMLAnchorElement;

  const response = await fetch(`/raw/${rawUrl.href}`, {
    method: "GET",
    headers: {
      "Content-Type": "text/plain",
    },
    mode: "cors",
    cache: "default",
  });
  if (response.status !== 200) return;
  const fileContent = await response.text();

  const language =
    document.getElementById("codeContainer")?.getAttribute("data-language") ??
    "";

  const data = new Blob([fileContent], {
    type: "text/plain",
  });
  const objectURL = window.URL.createObjectURL(data);
  const downloadLink = document.createElement("a");
  downloadLink.style.display = "none";
  downloadLink.href = objectURL;
  downloadLink.download = `${window.location.pathname.split("/")[1]}.${
    languageExtensions[language] ?? "txt"
  }`;
  downloadLink.click();
  window.URL.revokeObjectURL(objectURL);
};
