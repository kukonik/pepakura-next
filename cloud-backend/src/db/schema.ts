import { drizzle } from 'drizzle-orm/node-postgres'
import { pgTable, text, timestamp, integer, jsonb, boolean, uuid } from 'drizzle-orm/pg-core'
import { relations } from 'drizzle-orm'

// Users table
export const users = pgTable('users', {
  id: uuid('id').primaryKey().defaultRandom(),
  email: text('email').notNull().unique(),
  password_hash: text('password_hash').notNull(),
  display_name: text('display_name'),
  avatar_url: text('avatar_url'),
  created_at: timestamp('created_at').defaultNow().notNull(),
  updated_at: timestamp('updated_at').defaultNow().notNull(),
})

// Projects table
export const projects = pgTable('projects', {
  id: uuid('id').primaryKey().defaultRandom(),
  user_id: uuid('user_id').notNull().references(() => users.id),
  name: text('name').notNull(),
  description: text('description'),
  thumbnail_url: text('thumbnail_url'),
  is_public: boolean('is_public').default(false).notNull(),
  created_at: timestamp('created_at').defaultNow().notNull(),
  updated_at: timestamp('updated_at').defaultNow().notNull(),
})

// Project versions table
export const project_versions = pgTable('project_versions', {
  id: uuid('id').primaryKey().defaultRandom(),
  project_id: uuid('project_id').notNull().references(() => projects.id),
  version: integer('version').notNull(),
  s3_key: text('s3_key').notNull(),
  file_size: integer('file_size').notNull(),
  commit_message: text('commit_message'),
  created_at: timestamp('created_at').defaultNow().notNull(),
})

// Project settings table
export const project_settings = pgTable('project_settings', {
  id: uuid('id').primaryKey().defaultRandom(),
  project_id: uuid('project_id').notNull().references(() => projects.id),
  settings: jsonb('settings').notNull(),
  updated_at: timestamp('updated_at').defaultNow().notNull(),
})

// Relations
export const usersRelations = relations(users, ({ many }) => ({
  projects: many(projects),
}))

export const projectsRelations = relations(projects, ({ one, many }) => ({
  user: one(users, {
    fields: [projects.user_id],
    references: [users.id],
  }),
  versions: many(project_versions),
  settings: one(project_settings),
}))

export const projectVersionsRelations = relations(project_versions, ({ one }) => ({
  project: one(projects, {
    fields: [project_versions.project_id],
    references: [projects.id],
  }),
}))

export const projectSettingsRelations = relations(project_settings, ({ one }) => ({
  project: one(projects, {
    fields: [project_settings.project_id],
    references: [projects.id],
  }),
}))

// Types
export type User = typeof users.$inferSelect
export type NewUser = typeof users.$inferInsert
export type Project = typeof projects.$inferSelect
export type NewProject = typeof projects.$inferInsert
export type ProjectVersion = typeof projectVersions.$inferSelect
export type NewProjectVersion = typeof projectVersions.$inferInsert
export type ProjectSettings = typeof projectSettings.$inferSelect
export type NewProjectSettings = typeof projectSettings.$inferInsert
