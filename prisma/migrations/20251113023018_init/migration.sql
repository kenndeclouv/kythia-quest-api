-- CreateTable
CREATE TABLE "CacheStore" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "data" JSONB NOT NULL,
    "updatedAt" DATETIME NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "CacheStore_id_key" ON "CacheStore"("id");
