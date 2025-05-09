import { useEffect, useState } from 'react'
import './Options.css'

const HOST = "127.0.0.1"
const PORT = 0xDEAD

export const Options = () => {
  const [status, setSatus] = useState<number | null>();

  useEffect(() => {
    setSatus(null);
    fetch(`http://${HOST}:${PORT}/check`).then(response => setSatus(response.status)).catch(() => setSatus(null))
  }, []);

  return (
    <main>
      <h1>{status === 200 ? "🟢 Connected" : "🔴 Not connected"}</h1>
    </main>
  )
}

export default Options
