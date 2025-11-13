import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import questsRouter from "./routes/quests.js";
import { execSync } from "child_process";

try {
  console.log("Checking database synchronization...");

  execSync("bunx prisma db push", { stdio: "inherit" });
  console.log("Database is ready! ✨");
} catch (error) {
  console.error("Failed to synchronize database:", error.message);

  process.exit(1);
}

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

app.use(cors());
app.use(express.json());

app.use("/v1/quests", questsRouter);

app.get("/health", (req, res) => {
  res.status(200).json({ status: "ok" });
});

app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({
    error: "Something went wrong!",
    message: err.message,
  });
});

app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
  console.log(`API available at http://localhost:${PORT}/v1/quests`);
});
