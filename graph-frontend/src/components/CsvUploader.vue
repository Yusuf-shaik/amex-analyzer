<template>
  <div>
    <div class="text-center mt-5">
      <div class="container mt-5">
  <div class="row">
    <div class="col-md-12">
      <div class="mb-3">
        <input class="form-control" type="file" id="formFile" @change="handleFileUpload">
      </div>
    </div>
  </div>
</div>
    </div>
    <div v-if="chartData" class="text-center mt-5" id="chart">
      <canvas ref="chart" class="border"></canvas>
    </div>
  </div>
</template>
  
  <script>
  import axios from "axios";
  import Chart from "chart.js/auto";
  
  export default {
    data() {
      return {
        file: null,
        chartData: null,
        chartInstance: null,
      };
    },
    methods: {
      handleFileUpload(event) {
        this.file = event.target.files[0];
        if (this.file) {
        this.processCSV();
      }
      },
      async processCSV() {
        if (!this.file) {
          alert("Please select a file first.");
          return;
        }
  
        const formData = new FormData();
        formData.append("file", this.file);
  
        try {
          const response = await axios.post(
            "http://127.0.0.1:8080/process_csv",
            formData,
            {
              headers: {
                "Content-Type": "multipart/form-data",
              },
            }
          );
  
          this.chartData = response.data.categories;
          if (this.chartInstance) {
            this.chartInstance.destroy();
          }
          console.info(this.chartData)
          
  
          // Display chart
          this.$nextTick(() => {
            this.renderChart();
          });
        } catch (error) {
          console.error("Error processing CSV:", error);
        }
      },
      renderChart() {
        const ctx = this.$refs.chart.getContext("2d");
  
        this.chartInstance = new Chart(ctx, {
          type: "bar",
          data: {
            labels: Object.keys(this.chartData),
            datasets: [
              {
                label: "Spending by Category",
                data: Object.values(this.chartData),
                backgroundColor: [
                "#FF847C", 
                "#5991c2",
                "#b7d4ab",
                "#D9BF77",
                "#B19CD9",
                "#D291BC",
                "#6b3a36",
                "#8ED2C9"
                ],
              },
            ],
          },
        });
      },
    },
  };
  </script>
  
  <style scoped>
  @import '~bootstrap/dist/css/bootstrap.min.css';

  body{
    background-color: #191b1c;
  }

    #chart {
      padding-right: 20rem;
      padding-left: 20rem;
      padding-top: 1rem;
    }

  </style>