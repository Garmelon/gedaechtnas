import { z } from "zod/v4";

export type NodeId = z.infer<typeof NodeId>;
export const NodeId = z.string().startsWith("n").length(17);

export type Note = z.infer<typeof Note>;
export const Note = z.object({
  id: NodeId,
  text: z.string(),
  children: z.array(NodeId),
  parents: z.array(NodeId).transform((it) => new Set(it)),
});

////////////
// Events //
////////////

export type EventNoteStoreUpdate = z.infer<typeof EventNoteStoreUpdate>;
export const EventNoteStoreUpdate = z.object({
  storeId: z.number(),
});
