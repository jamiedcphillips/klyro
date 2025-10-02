import { useEffect, useState } from "react";
import { API } from "../lib/api";
import MediaCard from "../components/MediaCard";

export default function Home() {
  const [items, setItems] = useState<any[]>([]);
  useEffect(() => {
    API.getContinue().then(setItems);
  }, []);
  return (
    <section>
      <h2>Continue Watching</h2>
      <div className="grid">
        {items.map((m) => <MediaCard key={m.id} item={m} />)}
      </div>
    </section>
  );
}