import { defineContentConfig, defineCollection, property } from "@nuxt/content";
import { z } from "zod/v4";

const commonSchema = z.object({
  title: z.string().optional(),
  description: z.string().optional(),
  updatedAt: z.date().optional(),
  draft: z.boolean().optional(),
  tags: z.array(z.string()).optional(),
  hero: z
    .object({
      image: property(z.string()).editor({ input: "media" }),
      caption: z.string().optional(),
    })
    .optional(),
});
export default defineContentConfig({
  collections: {
    // English content collection
    content_en: defineCollection({
      type: "page",
      source: {
        include: "en/**",
        prefix: "",
      },
      schema: commonSchema,
    }),
    // French content collection
    content_fr: defineCollection({
      type: "page",
      source: {
        include: "fr/**",
        prefix: "",
      },
      schema: commonSchema,
    }),
    // Japanese content collection
    content_ja: defineCollection({
      type: "page",
      source: {
        include: "ja/**",
        prefix: "",
      },
      schema: commonSchema,
    }),
    // Korean content collection
    content_ko: defineCollection({
      type: "page",
      source: {
        include: "ko/**",
        prefix: "",
      },
      schema: commonSchema,
    }),
    // Traditional Chinese content collection
    content_zhHant: defineCollection({
      type: "page",
      source: {
        include: "zhHant/**",
        prefix: "",
      },
      schema: commonSchema,
    }),
    // Simplified Chinese content collection
    content_zhHans: defineCollection({
      type: "page",
      source: {
        include: "zhHans/**",
        prefix: "",
      },
      schema: commonSchema,
    }),
  },
});
