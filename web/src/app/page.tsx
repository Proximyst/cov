"use client";

import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { useState } from "react";

function Header() {
  // This is set up such that the top of the screen has:
  //
  // [orgimg] [org dropdown]                       [admin btn] [dark/light toggle] [help dropdown] [user img]
  const [selectedOrg, setSelectedOrg] = useState("todo"); // TODO: Use a context for this to propagate throughout the app

  return (
    <div className="container mx-auto px-4 md:px-6 lg:px-8">
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
          <Button variant="outline">Admin</Button>
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

function RepositoryCard({ name }: { name?: string }) {
  return (
    <Card className="w-64 h-64">
      <CardHeader>
        <CardTitle>{name ?? "repo here"}</CardTitle>
      </CardHeader>
    </Card>
  );
}

function RepositoryList() {
  return (
    <div className="p-8 flex flex-row gap-8 align-top">
      <RepositoryCard name="repo 1" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="very-long-repository-name-here-thats-wild-isnt-it-wow-i-should-recite-a-poem" />
    </div>
  );
}

function RepositoryDetails() {
  return (
    <div className="p-8 flex flex-row align-top container mr-8">
      this will be the right side
    </div>
  );
}

export default function Overview() {
  return (
    <div className="flex flex-col gap-4 min-h-screen">
      <div className="min-h-auto">
        <Header />
      </div>

      <div className="flex flex-row gap-2">
        <div className="flex flex-col">
          Test
          <Separator orientation="horizontal" className="my-4 h-64" />
          <RepositoryList />
        </div>
        <Separator orientation="vertical" className="my-4 h-64" />
        <div className="flex flex-col">
          Test
          <Separator orientation="horizontal" className="my-4 h-64" />
          <RepositoryDetails />
        </div>
      </div>
    </div>
  );
}
