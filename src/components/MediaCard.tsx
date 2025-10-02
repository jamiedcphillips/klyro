import { useNavigate } from "react-router-dom";

export default function MediaCard({ item }: { item: any }) {
  const nav = useNavigate();
  return (
    <div className="card" onClick={() => nav(`/movies?play=${item.id}`)}>
      <div className="poster" style={{ backgroundImage: `url(${item.poster_url || ''})`}} />
      <div className="meta">
        <div className="title">{item.title}</div>
      </div>
    </div>
  );
}