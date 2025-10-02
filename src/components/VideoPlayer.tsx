import { useEffect, useRef, useState } from "react";
import Hls from "hls.js";
import { API } from "../lib/api";

export default function VideoPlayer({ mediaId, onClose }: { mediaId: string, onClose: ()=>void }) {
  const ref = useRef<HTMLVideoElement>(null);
  const [duration, setDuration] = useState(0);

  useEffect(() => {
    (async () => {
      const url = await API.startStream(mediaId);
      const video = ref.current!;
      if (Hls.isSupported()) {
        const hls = new Hls();
        hls.loadSource(url);
        hls.attachMedia(video);
      } else {
        video.src = url;
      }
    })();
  }, [mediaId]);

  useEffect(() => {
    const v = ref.current!;
    const onTime = () => API.updateProgress(mediaId, Math.floor(v.currentTime), Math.floor(v.duration || duration)).catch(()=>{});
    const iv = setInterval(onTime, 5000);
    return () => clearInterval(iv);
  }, [mediaId, duration]);

  return (
    <div className="player-modal">
      <video ref={ref} controls autoPlay onLoadedMetadata={e => setDuration((e.target as HTMLVideoElement).duration)} />
      <button onClick={onClose}>Close</button>
    </div>
  );
}