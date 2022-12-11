import express from 'express'
import Queue from './queue'

const app = express()
const port = Number(process.env.PORT || 3000)
const queue = new Queue()

function clearQueue() {
  const now = Date.now()
  while (queue.peek() && queue.peek()!.ttl < now) {
    queue.pop()
  }
}

app.use(express.json())
app.use((_req, _res, next) => {
  clearQueue()
  next()
})

app.get('/status', (_req, res) => {
  res.send(`Queue size: ${queue.size}`)
})

app.post('/push/:ttl', (req, res) => {
  const { ttl } = req.params
  const json = req.body
  queue.push(json, Date.now() + Number(ttl))
  res.send()
})

app.listen(port, '0.0.0.0', () => {
  console.log(`Server running on port ${port}`)
})
