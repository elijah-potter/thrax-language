<script lang="ts">
  import { Context } from "wasm"

  let code = ""

  function runCode(){
    let context = new Context();

    context.easy_eval(code);
  }
</script>


<textarea bind:value={code}/>
<button on:click={runCode}>Run</button>
