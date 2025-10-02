import { useEffect, useState } from "react";
import { API } from "../lib/api";
import VideoPlayer from "../components/VideoPlayer";

export default function Movies() {
  const [movies, setMovies] = useState<any[]>([]);
  const [playing, setPlaying] = useState<string | null>(null);
  useEffect(() => { API.getMovies().then(setMovies); }, []);
  return (
    <section>
      <h2>Movies</h2>
      <div className="grid">
        {movies.map(m => (
          <div key={m.id} className="card" onClick={() => setPlaying(m.id)}>
            <div className="poster" />
            <div className="meta"><div className="title">{m.title}</div></div>
          </div>
        ))}
      </div>
      {playing && <VideoPlayer mediaId={playing} onClose={() => setPlaying(null)} />}
    </section>
  );
}