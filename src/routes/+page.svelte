<script>
  import { onMount } from 'svelte';
  import Meyda from 'meyda';

  let canvas;
  let canvasCtx;
  let audioCtx;
  let source;
  let meydaAnalyzer;
  let stream;

  let isRecording = false;
  let currentNote = "-";
  
  const NOTE_NAMES = ["Do", "Do#", "Ré", "Ré#", "Mi", "Fa", "Fa#", "Sol", "Sol#", "La", "La#", "Si"];

  function analyzeAudioData(features) {
    const { amplitudeSpectrum, chroma } = features;

    let maxChromaValue = 0;
    let noteIndex = -1;

    for (let i = 0; i < chroma.length; i++) {
      if (chroma[i] > maxChromaValue) {
        maxChromaValue = chroma[i];
        noteIndex = i;
      }
    }

    if (maxChromaValue > 0.4) {
      currentNote = NOTE_NAMES[noteIndex];
    } else {
      currentNote = "-";
    }

    drawSpectrogram(amplitudeSpectrum);
  }

  function drawSpectrogram(amplitudeSpectrum) {
    if (!canvasCtx) return;

    const width = canvas.width;
    const height = canvas.height;
    const speed = 2;

    const imageData = canvasCtx.getImageData(speed, 0, width - speed, height);
    canvasCtx.putImageData(imageData, 0, 0);

    canvasCtx.fillStyle = '#111';
    canvasCtx.fillRect(width - speed, 0, speed, height);

    const displayLen = Math.floor(amplitudeSpectrum.length * 0.5); 

    for (let i = 0; i < displayLen; i++) {
      const value = amplitudeSpectrum[i];
      const percent = Math.min(value / 10, 1);
      const y = height - (i / displayLen) * height;

      if (value > 0.1) {
        const r = Math.round(percent * 255);
        const g = Math.round(Math.max(0, (percent - 0.5) * 2 * 255));
        const b = Math.round((1 - percent) * 255);

        canvasCtx.fillStyle = `rgb(${r}, ${g}, ${b})`;
        canvasCtx.fillRect(width - speed, y, speed, 2);
      }
    }
  }

  async function startAudio() {
    try {
      stream = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
      audioCtx = new (window.AudioContext || window.webkitAudioContext)();
      source = audioCtx.createMediaStreamSource(stream);

      canvasCtx = canvas.getContext('2d');
      isRecording = true;

      meydaAnalyzer = Meyda.createMeydaAnalyzer({
        audioContext: audioCtx,
        source: source,
        bufferSize: 1024,
        featureExtractors: ['amplitudeSpectrum', 'chroma'],
        callback: (features) => {
          analyzeAudioData(features);
        },
      });

      meydaAnalyzer.start();

    } catch (err) {
      console.error("Microphone error:", err);
      alert("Please enable microphone access in your system.");
    }
  }

  function stopAudio() {
    isRecording = false;
    if (meydaAnalyzer) meydaAnalyzer.stop();
    if (stream) stream.getTracks().forEach(track => track.stop());
    if (audioCtx) audioCtx.close();
    currentNote = "-";
  }

  onMount(() => {
    return () => stopAudio();
  });
</script>

<main class="container">
  <div class="controls">
    {#if !isRecording}
      <button on:click={startAudio} class="btn-start">Open Microphone</button>
    {:else}
      <button on:click={stopAudio} class="btn-stop">Stop</button>
    {/if}
  </div>

  <div class="result-box">
    <h2>Detected Note: <span class="note">{currentNote}</span></h2>
  </div>

  <div class="visualizer">
    <canvas bind:this={canvas} width="600" height="300"></canvas>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, sans-serif;
    background-color: #121212;
    color: #e0e0e0;
  }
  .container { 
    display: flex; 
    flex-direction: column; 
    align-items: center; 
    padding: 20px; 
  }
  .controls { margin: 20px 0; }
  button { 
    padding: 12px 24px; 
    font-size: 16px; 
    border: none; 
    border-radius: 6px; 
    cursor: pointer; 
    font-weight: bold; 
  }
  .btn-start { 
    background-color: #007acc; 
    color: white; 
  }
  .btn-stop { 
    background-color: #d32f2f; 
    color: white; 
  }
  .result-box { 
    text-align: center; 
    margin-bottom: 20px; 
    background: #1e1e1e; 
    padding: 15px 40px; 
    border-radius: 8px; 
    border: 1px solid #333; 
  }
  .note { 
    color: #00ffcc; 
    font-size: 2em; 
    font-weight: bold; 
  }
  .visualizer { 
    background: #000; 
    border: 2px solid #333; 
    border-radius: 4px; 
  }
  canvas { display: block; }
</style>