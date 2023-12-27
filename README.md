# Amex Analyzer


Upload your Amex statement to our site and have AI Categorize your spending! 
1. Download your amex statement (csv) from [amex.ca](https://www.americanexpress.com/en-ca/)
2. Upload it to our site
3. Receive an interactive breakdown of your spending, categorized by AI!

### Examples:

![image](https://github.com/Yusuf-shaik/amex-analyzer/assets/43193906/31f82e14-546c-48d0-9988-6d21a547059c)

![image](https://github.com/Yusuf-shaik/amex-analyzer/assets/43193906/c3b4dbf9-7fa3-406b-b936-a27e44311618)



## Prerequisites

Before you begin, ensure you have the following:

- [Install Docker](https://www.docker.com/)
- [Get OpenAI API Key](https://platform.openai.com/api-keys)


## Getting Started

### 1. Clone the Repository

```bash
git clone git@github.com:Yusuf-shaik/amex-analyzer.git
cd amex-analyzer
```
### 2. Add OpenAI API Key as Environment Variable
 - Create a .env file in the `Amex-Analyzer/Backend` directory
 - Set OPENAI_API_KEY="<your_api_key_here>"

### 3. Build and Run
 - Run `docker-compose up --build`
 - Application will be available at http://localhost:8081/
 - Backend health can be verified with `curl http://localhost:8080`
