import { useEffect, useState } from "react";

export default function useCountDown() {
  const [countdown, setCountdown] = useState(0);

  useEffect(() => {
    setCountdown(60);
  }, []);

  useEffect(() => {
    if (countdown <= 0) return;

    const timeout = setTimeout(() => {
      setCountdown(countdown - 1);
    }, 1000);

    return () => clearInterval(timeout);
  }, [countdown]);

  function start(newTimer: number) {
    setCountdown(newTimer);
  }
  return { countdown, start };
}
