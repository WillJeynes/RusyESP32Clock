import { FC } from "react";
import { Link } from "react-router-dom";

interface SidebarLinkProps {
  link: string;
  text: string;
}

const OvSidebarLink: FC<SidebarLinkProps> = ({ link, text }) => {
  return (
    <Link to={link} className=" hover:bg-gray-500 transition-all rounded-xs p-2"><span className="text-white">{text}</span></Link>
  );
};

export default OvSidebarLink;
