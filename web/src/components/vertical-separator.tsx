// This is a patched version of @/components/ui/separator.
"use client";

import * as SeparatorPrimitive from "@radix-ui/react-separator";
import type * as React from "react";

import { cn } from "@/lib/utils";

export function VerticalSeparator({
  className,
  decorative = true,
  ...props
}: Omit<React.ComponentProps<typeof SeparatorPrimitive.Root>, "orientation">) {
  return (
    <SeparatorPrimitive.Root
      data-slot="separator-root"
      decorative={decorative}
      orientation="vertical"
      className={cn(
        // PATCH: https://github.com/shadcn-ui/ui/issues/4818
        // We use min-h-full instead of h-full.
        "bg-border shrink-0 data-[orientation=vertical]:min-h-full data-[orientation=vertical]:w-px",
        className
      )}
      {...props}
    />
  );
}
