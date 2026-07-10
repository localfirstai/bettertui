import { defineCollection } from "astro:content";
import { docsLoader } from "@astrojs/starlight/loaders";
import { docsSchema } from "@astrojs/starlight/schema";
import { z } from "astro/zod";

const docs = defineCollection({ loader: docsLoader(), schema: docsSchema() });

const blog = defineCollection({
  type: "content",
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    author: z.string().default("BetterTUI Team"),
    tags: z.array(z.string()).default([]),
    image: z.string().optional(),
  }),
});

const changelog = defineCollection({
  type: "content",
  schema: z.object({
    version: z.string(),
    date: z.coerce.date(),
    tags: z.array(z.string()).default([]),
  }),
});

export const collections = { docs, blog, changelog };
