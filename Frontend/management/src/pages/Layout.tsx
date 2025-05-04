import { FC } from "react";
import { Outlet } from "react-router-dom";
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarTrigger } from "../components/ui/sidebar";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "../components/ui/collapsible"
import { ChevronDown } from "lucide-react";
import OvSidebarGroup from "../components/overrides/OvSidebarGroup";
import OvSidebarLink from "../components/overrides/OvSidebarLink";
import OvSidebarSectionTitle from "../components/overrides/OvSidebarSectionTitle";
import { NavUser } from "../components/NavUser";
import { Separator } from "../components/ui/separator";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from "../components/ui/breadcrumb";
import { Avatar, AvatarFallback, AvatarImage } from "@radix-ui/react-avatar";

const Layout: FC = () => {
  return (
    <SidebarProvider className=" min-w-[100vw] flex">
      <Sidebar>
        <SidebarHeader>
          <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
            <Avatar className="h-8 w-8 rounded-lg">
              <AvatarImage src="https://picsum.photos/100/100" alt="Site Logo" />
              <AvatarFallback className="rounded-lg">RC</AvatarFallback>
            </Avatar>
            <div className="grid flex-1 text-left text-sm leading-tight">
              <span className="truncate font-semibold">ManagementUI</span>
              <span className="truncate text-xs">v1.1</span>
            </div>
          </div>
        </SidebarHeader>
        <SidebarContent>
          <OvSidebarLink link="/" text="Home" />
          <OvSidebarGroup>
            <OvSidebarSectionTitle>Devices</OvSidebarSectionTitle>
            <OvSidebarLink link="/devices" text="Clocks" />
          </OvSidebarGroup>
          <OvSidebarGroup grow>
            <OvSidebarSectionTitle>Images</OvSidebarSectionTitle>
            <OvSidebarLink link="/images" text="My Images" />
            <OvSidebarLink link="/images/new" text="Create Image" />
          </OvSidebarGroup>
          <Collapsible className="group/collapsible" >
            <SidebarGroup>
              <SidebarGroupLabel asChild>
                <CollapsibleTrigger >
                  Settings
                  <ChevronDown className="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-180" />
                </CollapsibleTrigger>
              </SidebarGroupLabel>
              <CollapsibleContent>
                <OvSidebarGroup>
                  <OvSidebarLink link="/settings/devices" text="Device Management" />
                  <OvSidebarLink link="/settings/devices" text="Image Sources" />
                </OvSidebarGroup>
              </CollapsibleContent>
            </SidebarGroup>
          </Collapsible>


        </SidebarContent>
        <SidebarFooter>
          <NavUser user={{ avatar: "https://picsum.photos/100/100", email: "e@e.e", name: "John Doe" }} />
        </SidebarFooter>

      </Sidebar>

      <div className="flex flex-col h-screen grow">
        
          < SidebarInset >
          <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <p className="text-2xl">Rusty Clock Management</p>
            <Separator orientation="vertical" className="mr-2 h-4" />
            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbItem className="hidden md:block">
                  <BreadcrumbLink href="#">
                    BreadcrumbSection
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator className="hidden md:block" />
                <BreadcrumbItem>
                  <BreadcrumbPage>BreadPage</BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>

          </header>
          <main className="grow overflow-auto ">
            <div className="p-5">
              <Outlet />
            </div>
          </main>

          </SidebarInset>
          
      </div>




    </SidebarProvider>
  )
};

export default Layout;