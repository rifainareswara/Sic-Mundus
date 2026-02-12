<template>
  <div>
    <div class="flex items-center justify-between mb-8">
      <div>
        <h2 class="text-3xl font-black text-white tracking-tight">Reports</h2>
        <p class="text-slate-500 mt-1">Generate monthly PDF reports for your tasks</p>
      </div>
    </div>

    <!-- Month Picker & Generate -->
    <div class="bg-slate-900 rounded-xl border border-slate-800 p-6 mb-6">
      <div class="flex flex-wrap items-end gap-4">
        <div>
          <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Month</label>
          <select v-model="selectedMonth" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
            <option v-for="m in months" :key="m.value" :value="m.value">{{ m.label }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-bold text-slate-500 uppercase tracking-wide mb-1.5">Year</label>
          <select v-model="selectedYear" class="bg-slate-950 border border-slate-700 rounded-lg px-4 py-2.5 text-sm text-slate-200 focus:ring-2 focus:ring-blue-500 outline-none">
            <option v-for="y in years" :key="y" :value="y">{{ y }}</option>
          </select>
        </div>
        <button
          class="px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold shadow-md hover:shadow-lg transition-all flex items-center gap-2"
          :disabled="generating"
          @click="generateReport"
        >
          <span v-if="generating">⏳ Generating...</span>
          <span v-else>📄 Generate PDF</span>
        </button>
      </div>
    </div>

    <!-- Preview Stats -->
    <div v-if="reportData" class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Total Tasks</div>
        <div class="text-2xl font-black text-white mt-1">{{ reportData.totalTasks }}</div>
      </div>
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Completed</div>
        <div class="text-2xl font-black text-emerald-400 mt-1">{{ reportData.completedTasks }}</div>
      </div>
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Time Entries</div>
        <div class="text-2xl font-black text-blue-400 mt-1">{{ reportData.totalEntries }}</div>
      </div>
      <div class="bg-slate-900 rounded-xl border border-slate-800 p-5">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wide">Total Hours</div>
        <div class="text-2xl font-black text-amber-400 mt-1">{{ reportData.totalHours }}</div>
      </div>
    </div>

    <!-- Task Preview Table -->
    <div v-if="reportData && reportData.tasks.length" class="bg-slate-900 rounded-xl border border-slate-800 overflow-hidden mb-6">
      <div class="px-6 py-4 border-b border-slate-800">
        <h3 class="font-bold text-slate-200">Tasks in {{ monthLabel }}</h3>
      </div>
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-slate-800 text-slate-500 text-xs uppercase tracking-wide">
            <th class="text-left px-6 py-3">Title</th>
            <th class="text-left px-6 py-3">Category</th>
            <th class="text-left px-6 py-3">Status</th>
            <th class="text-right px-6 py-3">Hours</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="task in reportData.tasks" :key="task.id" class="border-b border-slate-800/50 hover:bg-slate-800/30">
            <td class="px-6 py-3 text-slate-200 font-medium">{{ task.title }}</td>
            <td class="px-6 py-3 text-slate-400">{{ task.category }}</td>
            <td class="px-6 py-3">
              <span
                class="px-2 py-0.5 rounded text-xs font-bold"
                :class="{
                  'bg-yellow-900/30 text-yellow-400': task.status === 'pending',
                  'bg-blue-900/30 text-blue-400': task.status === 'in_progress',
                  'bg-emerald-900/30 text-emerald-400': task.status === 'completed',
                }"
              >{{ task.status }}</span>
            </td>
            <td class="px-6 py-3 text-right text-slate-300 font-mono">{{ task.hours }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Empty State -->
    <div v-if="reportData && !reportData.tasks.length" class="bg-slate-900 rounded-xl border border-slate-800 p-12 text-center">
      <div class="text-4xl mb-3 opacity-10">📋</div>
      <p class="text-slate-600">No data for {{ monthLabel }}.</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useTaskStore } from '~/stores/taskStore'

const store = useTaskStore()
const generating = ref(false)
const reportData = ref(null)

const now = new Date()
const selectedMonth = ref(now.getMonth())
const selectedYear = ref(now.getFullYear())

const months = [
  { value: 0, label: 'January' }, { value: 1, label: 'February' },
  { value: 2, label: 'March' }, { value: 3, label: 'April' },
  { value: 4, label: 'May' }, { value: 5, label: 'June' },
  { value: 6, label: 'July' }, { value: 7, label: 'August' },
  { value: 8, label: 'September' }, { value: 9, label: 'October' },
  { value: 10, label: 'November' }, { value: 11, label: 'December' },
]

const years = computed(() => {
  const cur = now.getFullYear()
  return [cur - 2, cur - 1, cur, cur + 1]
})

const monthLabel = computed(() => {
  return `${months[selectedMonth.value].label} ${selectedYear.value}`
})

onMounted(async () => {
  await store.fetchTasks()
  await store.fetchAllEntries()
})

function getMonthData() {
  const y = selectedYear.value
  const m = selectedMonth.value
  const startOfMonth = new Date(y, m, 1)
  const endOfMonth = new Date(y, m + 1, 0, 23, 59, 59)

  // Filter entries for the selected month
  const monthEntries = (store.allEntries || []).filter(e => {
    const d = new Date(e.start_time)
    return d >= startOfMonth && d <= endOfMonth
  })

  // Build per-task minutes map
  const taskMinutes = {}
  monthEntries.forEach(e => {
    const tid = e.task_id
    taskMinutes[tid] = (taskMinutes[tid] || 0) + (e.duration_minutes || 0)
  })

  // Get tasks that had entries or were created this month
  const tasks = (store.tasks || []).filter(t => {
    const created = new Date(t.created_at)
    return taskMinutes[t.id] || (created >= startOfMonth && created <= endOfMonth)
  }).map(t => ({
    id: t.id,
    title: t.title,
    category: t.category || 'General',
    status: t.status,
    minutes: taskMinutes[t.id] || 0,
    hours: ((taskMinutes[t.id] || 0) / 60).toFixed(1),
  }))

  const totalMinutes = Object.values(taskMinutes).reduce((a, b) => a + b, 0)

  return {
    tasks,
    entries: monthEntries,
    totalTasks: tasks.length,
    completedTasks: tasks.filter(t => t.status === 'completed').length,
    totalEntries: monthEntries.length,
    totalHours: (totalMinutes / 60).toFixed(1),
    totalMinutes,
  }
}

async function generateReport() {
  generating.value = true

  await store.fetchTasks()
  await store.fetchAllEntries()

  const data = getMonthData()
  reportData.value = data

  // Dynamic import for client-side only
  const { jsPDF } = await import('jspdf')
  const autoTableModule = await import('jspdf-autotable')

  const doc = new jsPDF()
  const pageW = doc.internal.pageSize.getWidth()

  // ── Header ──
  doc.setFillColor(15, 23, 42) // slate-900
  doc.rect(0, 0, pageW, 40, 'F')
  doc.setTextColor(255, 255, 255)
  doc.setFontSize(22)
  doc.setFont('helvetica', 'bold')
  doc.text('Sic Mundus', 14, 20)
  doc.setFontSize(10)
  doc.setFont('helvetica', 'normal')
  doc.setTextColor(148, 163, 184) // slate-400
  doc.text('Monthly Time Report', 14, 28)
  doc.setFontSize(12)
  doc.setTextColor(96, 165, 250) // blue-400
  doc.text(monthLabel.value, 14, 36)

  // ── Summary Stats ──
  let y = 50
  doc.setTextColor(30, 41, 59) // slate-800
  doc.setFontSize(11)
  doc.setFont('helvetica', 'bold')
  doc.text('Summary', 14, y)
  y += 8

  doc.setFont('helvetica', 'normal')
  doc.setFontSize(10)
  doc.setTextColor(71, 85, 105) // slate-600
  const summaryItems = [
    `Total Tasks: ${data.totalTasks}`,
    `Completed: ${data.completedTasks}`,
    `Time Entries: ${data.totalEntries}`,
    `Total Hours: ${data.totalHours}h`,
  ]
  summaryItems.forEach(item => {
    doc.text(item, 14, y)
    y += 6
  })

  // ── Tasks Table ──
  if (data.tasks.length) {
    y += 6
    doc.setFont('helvetica', 'bold')
    doc.setFontSize(11)
    doc.setTextColor(30, 41, 59)
    doc.text('Tasks', 14, y)
    y += 4

    doc.autoTable({
      startY: y,
      head: [['Title', 'Category', 'Status', 'Hours']],
      body: data.tasks.map(t => [t.title, t.category, t.status, `${t.hours}h`]),
      theme: 'striped',
      headStyles: { fillColor: [30, 41, 59], textColor: [255, 255, 255], fontStyle: 'bold', fontSize: 9 },
      bodyStyles: { fontSize: 9, textColor: [51, 65, 85] },
      alternateRowStyles: { fillColor: [241, 245, 249] },
      margin: { left: 14, right: 14 },
    })
    y = doc.lastAutoTable.finalY + 10
  }

  // ── Time Entries Table ──
  if (data.entries.length) {
    doc.setFont('helvetica', 'bold')
    doc.setFontSize(11)
    doc.setTextColor(30, 41, 59)
    doc.text('Time Entries', 14, y)
    y += 4

    const entryRows = data.entries.map(e => {
      const task = store.tasks.find(t => t.id === e.task_id)
      const date = new Date(e.start_time).toLocaleDateString('en-US', { day: 'numeric', month: 'short', year: 'numeric' })
      return [date, task?.title || '-', `${e.duration_minutes} min`, e.notes || '-']
    })

    doc.autoTable({
      startY: y,
      head: [['Date', 'Task', 'Duration', 'Notes']],
      body: entryRows,
      theme: 'striped',
      headStyles: { fillColor: [30, 41, 59], textColor: [255, 255, 255], fontStyle: 'bold', fontSize: 9 },
      bodyStyles: { fontSize: 8, textColor: [51, 65, 85] },
      alternateRowStyles: { fillColor: [241, 245, 249] },
      margin: { left: 14, right: 14 },
      columnStyles: { 3: { cellWidth: 60 } },
    })
  }

  // ── Footer ──
  const pageCount = doc.internal.getNumberOfPages()
  for (let i = 1; i <= pageCount; i++) {
    doc.setPage(i)
    doc.setFontSize(8)
    doc.setTextColor(148, 163, 184)
    doc.text(`Sic Mundus — ${monthLabel.value} — Page ${i}/${pageCount}`, pageW / 2, doc.internal.pageSize.getHeight() - 10, { align: 'center' })
  }

  // ── Save ──
  const filename = `SicMundus_Report_${months[selectedMonth.value].label}_${selectedYear.value}.pdf`
  doc.save(filename)
  generating.value = false
}
</script>
