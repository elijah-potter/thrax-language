<script lang="ts">
  import { Button, Card, Listgroup } from "flowbite-svelte";
  import { Terminal } from "xterm";
  import "xterm/css/xterm.css";
  import "./app.css";
  import { Context } from "wasm";
  import { onMount } from "svelte";
  import { AceEditor } from "svelte-ace";
  import { FitAddon } from "xterm-addon-fit";
  import demoItems from "./demoItems";
  import {WebglAddon} from "xterm-addon-webgl"

  function getqsv(param) {
    try {
      var qs = window.location.search.substring(1);
      var v = qs.split("&");
      for (var i = 0; i < v.length; i++) {
        var p = v[i].split("=");
        let pk = p.shift();
        let pn = p.reduce((vo, vn) => vo + vn, "");
        if (pk == param) {
          let parsed = decodeURIComponent(pn);
          if (typeof parsed === "string") {
            return parsed;
          }
        }
      }
    } catch (e) {
      console.log(e);
    }
    return null;
  }

  let code = {inner: getqsv("code") ?? ""};

  let webglAddon = new WebglAddon();
  let fitAddon = new FitAddon();
  let termDiv;
  let terminal;

  $: if (code.inner.length > 0) {
    history.pushState(null, null, `/?code=${encodeURIComponent(code.inner)}`);
  } else {
    history.pushState(null, null, "/");
  }

  onMount(() => {
    terminal = new Terminal();
    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webglAddon)
    terminal.open(termDiv);

    fitAddon.fit();
    window.addEventListener("resize", () => fitAddon.fit());

    window.addEventListener("keydown", (e) => {
      if (e.ctrlKey && e.code === "Enter"){
        runCode()
      }
    })
  });

  function runCode() {
    let context = new Context((t) => terminal.write(t + "\n\r"));

    try {
      terminal.write(`Program returned: ${context.easy_eval(code.inner)}\n\r`);
    } catch (e) {
      terminal.write(`\x1B[1;3;31m${e.toString()}\n\r\x1B[0m`);
    }
  }
</script>

<Card class="absolute right-6 top-6 z-20">
  <h3 class="font-bold text-lg pb-3">Example Scripts</h3>
  <Listgroup active items={demoItems} let:item class="w-48">
    {item["name"]}
  </Listgroup>
</Card>

<div class="flex flex-col w-full h-full">
  <div class="flex-shrink-0 flex-grow basis-9/12">
    <AceEditor bind:value={code.inner} />
  </div>
  <div class="flex-auto flex flex-row justify-end p-3" title="Run (CTRL + Enter)">
      <Button on:click={runCode}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 30 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z" />
        </svg>
        Run
      </Button>
  </div>
  <div bind:this={termDiv} class="flex-auto bg-black"/>
</div>
