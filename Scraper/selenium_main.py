import time
import pandas as pd
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.service import Service as FirefoxService
from selenium.webdriver.firefox.options import Options

# Configure Selenium WebDriver (using Firefox)
firefox_options = Options()
firefox_service = FirefoxService('geckodriver.exe')  # Specify the path to your geckodriver
driver = webdriver.Firefox(service=firefox_service, options=firefox_options)

# Add your manually extracted cookies here (replace with actual values)
cookies = [
    {}
    ]

# Navigate to the site
url = ""
driver.get(url)

# Inject the cookies
for cookie in cookies:
    driver.add_cookie(cookie)

# Reload the page with cookies set
driver.get(url)
time.sleep(20)  # Adjust if necessary

# Now proceed with scraping after the CAPTCHA has been bypassed
# Example: Scrape the table with market data (modify according to the actual structure)
rows = driver.find_elements(By.CSS_SELECTOR, '.ds-dex-table')  # Update selector based on actual structure
# Parse data
data = []
print(f"Found {len(rows)} rows.")
for row in rows:
    for index in range(2,102):
        try:
            pair_name = row.find_element(By.CSS_SELECTOR, f'a.ds-dex-table-row:nth-child({index}) > div:nth-child(1) > span:nth-child(4)').text  # Adjust selectors
            price = row.find_element(By.CSS_SELECTOR, f'a.ds-dex-table-row:nth-child({index}) > div:nth-child(2)').text  # Adjust selectors
            volume_24h = row.find_element(By.CSS_SELECTOR, f'a.ds-dex-table-row:nth-child({index}) > div:nth-child(3)').text  # Adjust selectors
        
            data.append({
                'Pair': pair_name,
                'Price': price,
                '24h Volume': volume_24h
            })
        except Exception as e:
            print(f"Error scraping row: {e}")

# Store data in DataFrame
# Get the current page's source after waiting
html = driver.page_source

# Write the HTML to a file for inspection
with open("page_source.html", "w", encoding="utf-8") as file:
    file.write(html)

print("HTML page source saved to 'page_source.html'.")

df = pd.DataFrame(data)
print(df)
# Save data to CSV
df.to_csv('data.csv', index=False)
# Clean up
driver.quit()

print("Scraping completed and data saved to data.csv.")

