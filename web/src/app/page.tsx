"use client";

import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { VerticalSeparator } from "@/components/vertical-separator";
import { useState } from "react";

function Header() {
  // This is set up such that the top of the screen has:
  //
  // [orgimg] [org dropdown]                       [admin btn] [dark/light toggle] [help dropdown] [user img]
  const [selectedOrg, setSelectedOrg] = useState("todo"); // TODO: Use a context for this to propagate throughout the app
  const isAdmin = true; // TODO: Propagate this somehow

  return (
    <div className="container mx-auto px-4 md:px-6 lg:px-8">
      <header className="flex justify-between h-20 w-full shrink-0 items-center px-4 md:px-6">
        <div className="flex items-center gap-4">
          <img src="/orgimg.png" alt="Organization logo" className="h-12 rounded-2xl" />
          <Select value={selectedOrg} onValueChange={(value) => setSelectedOrg(value)}>
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
              <img src="/userimg.jpeg" alt="User profile" className="h-12 rounded-2xl hover:outline-1" />
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

function RepositoryCard({ name }: { name?: string }) {
  return (
    <Card className="w-64 not-md:w-48 h-64 not-md:h-48">
      <CardHeader>
        <CardTitle>{name ?? "repo here"}</CardTitle>
      </CardHeader>
    </Card>
  );
}

function RepositoryListOptions() {
  return <>Repositories | Settings | Search (TODO)</>;
}

function RepositoryList() {
  return (
    <div className="grid grid-cols-2 not-md:grid-cols-1 p-8 gap-8 align-top">
      <RepositoryCard name="repo 1" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="very-long-repository-name-here-thats-wild-isnt-it-wow-i-should-recite-a-poem" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="repo 2" />
    </div>
  );
}

function RepositoryDetailsOptions() {
  return <>Coverage | Commits | PRs | Settings (TODO)</>;
}

function RepositoryDetails() {
  return (
    <div className="flex flex-col p-8 mr-8">
      <div className="grow">this will be the right side</div>
    </div>
  );
}

export default function Overview() {
  return (
    <div className="flex flex-col gap-4 min-h-screen">
      <div className="min-h-auto">
        <Header />
      </div>

      <div className="grow flex flex-row gap-0 px-4 pb-4 min-h-full">
        <div className="shrink flex flex-col">
          <RepositoryListOptions />
          <Separator orientation="horizontal" />
          <RepositoryList />
        </div>
        <VerticalSeparator className="shrink h-auto" />
        <div className="grow flex flex-col">
          <RepositoryDetailsOptions />
          <Separator orientation="horizontal" />
          <RepositoryDetails />
        </div>
      </div>
    </div>
  );
}
