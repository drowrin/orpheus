<script lang="ts" module>
  export type LineGraphPoint = {
    x: number
    normalize?: number
  } & Record<string, number>

  export type LineGraphProps = {
    data: LineGraphPoint[]
    colors: Record<string, string>
    padding?: {top?: number, right?: number, bottom?: number, left?: number}
  }
</script>

<script lang="ts">
  import { LayerCake, ScaledSvg, Html, groupLonger, flatten } from 'layercake';
  import { scaleOrdinal } from 'd3-scale';

  import MultiLine from "./MultiLine.svelte";
  import AxisX from "./AxisX.svelte";
  import AxisY from "./AxisY.svelte";
  import GroupLabels from './GroupLabels.svelte';

  const props: LineGraphProps = $props()

  const padding = $derived({
    top: 15,
    right: 5,
    bottom: 20,
    left: 10,
    ...props.padding,
  })

  const data = $derived(props.data.map((d) => {
    if (d.normalize === undefined) {
      return d
    }

    return {
      x: d.x,
      ...Object.fromEntries(
        Object.entries(d)
          .filter(([k, _]) => k !== 'x' && k !=='normalize')
          .map(([k, v]) => [k, v / d.normalize!])
      )
    }
  }))

  const seriesNames = $derived(Object.keys(data[0]).filter(d => d !== 'x'))
  const seriesColors = $derived(seriesNames.map(n => props.colors[n]))

  const groupedData = $derived(groupLonger(data, seriesNames, {valueTo: 'value', groupTo: 'cat'}))
</script>

<style>
  .chart-container {
    width: calc(100% - 2rem);
    height: 250px;
    margin-inline: 1rem;
  }
</style>

<div class="chart-container">
  <LayerCake
    ssr
    percentRange
    padding={padding}
    x={'x'}
    y={'value'}
    z={'cat'}
    yDomain={[0, null]}
    zScale={scaleOrdinal()}
    zRange={seriesColors}
    flatData={flatten(groupedData, 'values')}
    data={groupedData}
  >
    <Html>
        <AxisX 
          gridlines={false}
          ticks={data.map(d => d['x']).sort((a, b) => a - b).filter((_, i) => i % 2 === 0)}
          snapLabels
          tickMarks
          baseline
        />
        <AxisY />
      <GroupLabels />
    </Html>
    <ScaledSvg>
      <MultiLine />
    </ScaledSvg>
  </LayerCake>
</div>