import { useEffect, useState } from "react";
import { API } from "../lib/api";

export default function Library() {
  const [paths, setPaths] = useState<string[]>([]);
  const [input, setInput] = useState("");

  const refresh = () => API.listLibraryPaths().then(setPaths);
  useEffect(() => { refresh(); }, []);

  const add = async () => {
    if (!input) return;
    await API.addLibraryPath(input);
    setInput("");
    refresh();
  };
  const scan = async () => { await API.scanLibrary(); alert("Scan queued"); };

  return (
    <section>
      <h2>Library</h2>
      <div className="row">
        <input placeholder="Add folder path" value={input} onChange={e => setInput(e.target.value)} />
        <button onClick={add}>Add</button>
        <button onClick={scan}>Scan</button>
      </div>
      <ul>{paths.map(p => <li key={p}>{p}</li>)}</ul>
    </section>
  );
}