import { BrowserRouter, Routes, Route } from "react-router-dom";
import SideNav from "./components/SideNav";
import Home from "./pages/Home";
import Movies from "./pages/Movies";
import Series from "./pages/Series";
import LiveTV from "./pages/LiveTV";
import Guide from "./pages/Guide";
import Addons from "./pages/Addons";
import Library from "./pages/Library";
import Watchlist from "./pages/Watchlist";
import Settings from "./pages/Settings";
import "./app.css";

export default function App() {
  return (
    <BrowserRouter>
      <div className="app">
        <SideNav/>
        <main className="main">
          <Routes>
            <Route path="/" element={<Home/>} />
            <Route path="/movies" element={<Movies/>} />
            <Route path="/series" element={<Series/>} />
            <Route path="/live" element={<LiveTV/>} />
            <Route path="/guide" element={<Guide/>} />
            <Route path="/addons" element={<Addons/>} />
            <Route path="/library" element={<Library/>} />
            <Route path="/watchlist" element={<Watchlist/>} />
            <Route path="/settings" element={<Settings/>} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}