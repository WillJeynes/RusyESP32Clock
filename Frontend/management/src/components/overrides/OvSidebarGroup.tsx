import { FC, ReactNode } from "react";

interface SidebarGroupProps {
  children: ReactNode;
  grow?: boolean
}

const OvSidebarGroup: FC<SidebarGroupProps> = ({ children, grow = false }) => {
  return (

      <div className={`pl-2 pr-2 flex flex-col ${grow? "grow" : ""}`}>
        {children}
      </div>
    

  );
};

export default OvSidebarGroup;
