import express from "express";
import axios from "axios";
import { generateHeaders } from "../utils/headers.js";
import prisma from "../db/client.js";

const router = express.Router();

const CACHE_KEY = "discord_quests";

const CACHE_DURATION_MS = 30 * 60 * 1000;

router.get("/", async (req, res, next) => {
  try {
    const token = process.env.DISCORD_TOKEN;
    if (!token) {
      return next(new Error("Discord token not configured on server"));
    }

    const cachedData = await prisma.CacheStore.findUnique({
      where: { id: CACHE_KEY },
    });

    let isStale = true;
    if (cachedData) {
      const timeDiff = new Date() - new Date(cachedData.updatedAt);
      if (timeDiff < CACHE_DURATION_MS) {
        isStale = false;
      }
    }

    if (cachedData && !isStale) {
      return res.json(cachedData.data);
    }

    const headers = generateHeaders(token);

    const response = await axios.get("https://discord.com/api/v10/quests/@me", {
      headers,
    });

    const questsData = response.data.quests ?? [];

    await prisma.CacheStore.upsert({
      where: { id: CACHE_KEY },
      create: {
        id: CACHE_KEY,
        data: questsData,
      },
      update: {
        data: questsData,
        updatedAt: new Date(),
      },
    });

    res.json(questsData);
  } catch (error) {
    console.error("Error fetching quests:", error.message);
    next(error);
  }
});

export default router;
