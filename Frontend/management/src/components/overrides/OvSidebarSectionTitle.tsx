import { FC, ReactNode } from "react";

interface SidebarSectionTitleProps {
  children: ReactNode;
}

const OvSidebarSectionTitle: FC<SidebarSectionTitleProps> = ({ children }) => {
  return (
    <div className=" text-gray-500 " style={{color: "color-mix(in oklab, var(--sidebar-foreground) 70%, transparent)"}}>{children}</div>
  );
};

export default OvSidebarSectionTitle;
