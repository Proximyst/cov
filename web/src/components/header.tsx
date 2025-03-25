"use client";

import { useState } from "react";
import { Button } from "./ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";

export function Header() {
  // This is set up such that the top of the screen has:
  //
  // [orgimg] [org dropdown]                       [admin btn] [dark/light toggle] [help dropdown] [user img]
  const [selectedOrg, setSelectedOrg] = useState("todo"); // TODO: Use a context for this to propagate throughout the app
  const isAdmin = true; // TODO: Propagate this somehow

  return (
    <div className="min-h-auto container mx-auto px-4 md:px-6 lg:px-8">
      <header className="flex justify-between h-20 w-full shrink-0 items-center px-4 md:px-6">
        <div className="flex items-center gap-4">
          <img
            src="/orgimg.png"
            alt="Organization logo"
            className="h-12 rounded-2xl"
          />
          <Select
            value={selectedOrg}
            onValueChange={(value) => setSelectedOrg(value)}
          >
            <SelectTrigger className="w-64">
              <SelectValue placeholder="TODO" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="todo">TODO</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div className="flex items-center gap-4">
          {isAdmin && <Button variant="outline">Admin</Button>}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline">?</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-24">
              <DropdownMenuItem>GitHub</DropdownMenuItem>
              <DropdownMenuItem>Wiki</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
          <DropdownMenu>
            <DropdownMenuTrigger>
              <img
                src="/userimg.jpeg"
                alt="User profile"
                className="h-12 rounded-2xl hover:outline-1"
              />
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-24">
              <DropdownMenuItem>Profile</DropdownMenuItem>
              <DropdownMenuItem>Settings</DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem>Log out</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </header>
    </div>
  );
}
