import { Link, Outlet } from "react-router-dom";

export default function Layout() {
  return (
    <>
      <Link to="/collections">Collections</Link>
      <Outlet />
    </>
  )
}