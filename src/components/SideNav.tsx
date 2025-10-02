import { NavLink } from "react-router-dom";

export default function SideNav() {
  const link = ({ isActive }: any) => "navlink" + (isActive ? " active" : "");
  return (
    <aside className="sidenav">
      <h1 className="logo">Klyro</h1>
      <nav>
        <NavLink to="/" className={link}>Home</NavLink>
        <NavLink to="/movies" className={link}>Movies</NavLink>
        <NavLink to="/series" className={link}>Series</NavLink>
        <NavLink to="/live" className={link}>Live TV</NavLink>
        <NavLink to="/guide" className={link}>Guide</NavLink>
        <NavLink to="/addons" className={link}>Add-ons</NavLink>
        <NavLink to="/library" className={link}>Library</NavLink>
        <NavLink to="/watchlist" className={link}>Watchlist</NavLink>
        <NavLink to="/settings" className={link}>Settings</NavLink>
      </nav>
    </aside>
  );
}