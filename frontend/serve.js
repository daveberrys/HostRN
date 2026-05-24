import express from 'express'
import path from 'path'
import { fileURLToPath } from 'url'
import 'dotenv/config'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const app = express()
const port = 4678

app.use(express.static(__dirname))
app.get('/config', (req, res) => {
    res.json({
        API_URL: process.env.PUBLIC_SERVER_IP
    });
});

app.listen(port, () => {
  console.log(`Server running at: http://localhost:${port}`)
})
