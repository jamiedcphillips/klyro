import express from "express";
import fetch from "node-fetch";
const app = express();
const TMDB_KEY = process.env.TMDB_KEY;

app.get("/manifest.json", (_, res) => res.json(JSON.parse(`${
  JSON.stringify(require('./manifest.json'))
}`)));

app.get("/catalog/:type/:id.json", async (req, res) => {
  const page = req.query.page || 1;
  const r = await fetch(`https://api.themoviedb.org/3/trending/movie/day?api_key=${TMDB_KEY}&page=${page}`);
  const j = await r.json();
  res.json({
    metas: j.results.map(m => ({
      id: `tmdb:${m.id}`, type: "movie", name: m.title, year: m.release_date?.slice(0,4),
      poster: m.poster_path ? `https://image.tmdb.org/t/p/w342${m.poster_path}` : null
    }))
  });
});

app.get("/meta/:type/:id.json", async (req, res) => {
  const id = req.params.id.split(":")[1];
  const r = await fetch(`https://api.themoviedb.org/3/movie/${id}?api_key=${TMDB_KEY}`);
  const m = await r.json();
  res.json({
    meta: {
      id: `tmdb:${m.id}`, type: "movie", name: m.title, year: m.release_date?.slice(0,4),
      overview: m.overview,
      poster: m.poster_path ? `https://image.tmdb.org/t/p/w500${m.poster_path}` : null,
      background: m.backdrop_path ? `https://image.tmdb.org/t/p/w1280${m.backdrop_path}` : null,
      runtime: (m.runtime || 0) * 60
    }
  });
});

const port = process.env.PORT || 7000;
app.listen(port, () => console.log("Addon on http://127.0.0.1:"+port));