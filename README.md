# Simple Personal Accounting

What are the tools in my simple-accounting-tool?

- [ ] Assets and Liabilities
- [ ] Profits and Losses
- [ ] Cash Flow Graph
- [ ] Scan Receipts ?
- [ ] Data Imports and Exports ?
- [ ] Expected Tax

## Database

SQLite with the possibility of hosting somewhere for increased security. Could find a way to encrypt the contents to the database while local.

## Making the Assets and Liabilities functions

- [ ] Base Structure
  * will be keeping copies of each Name to have idea of how many 
  - [ ] Input Date 

- [ ] Assets Data Structure
  - [ ] Name
  - [ ] Type (real estate, checking, saving, stocks, bonds, car, possessions, etc)
  - [ ] Total Valuation (owning amount)

- [ ] Liabilities Data Structure
  - [ ] Name
  - [ ] Type (loans, rent, food, renovations, clothing, insurance, subscriptions, )
  - [ ] Recurring (yes or no)
  - [ ] Frequency (yearly, quarterly, bi-yearly, monthly, weekly, daily)
  - [ ] Valuation (total or  per-frequency)


Command: Either create assets or create liabilities 
Command: Either read assets or read Liabilities (filter by type)

Should have the option to visualize what potential assets and liabilities can add or detract to or from the accounts. For example: How would a raise today affect my balance; in 1 month, 1 year? How would that subscription compare to my balance with and without it; in 1 month, 1 year? At the current cost of my liabilities, how would my account compare in 1 month, 1 year?

The display should dynamically increase and decrease in input fields based on user input. Having an undo/redo button would be nice. The form should pre-populate with the data in the database or zeros. The visualization tool should update based on form inputs.
### Necessary Components for the above
- [ ] Plug-able projections system
- [ ] A visualization tool

#### Visualization tool
It should update based on the inputs of the data. Tabs containing the variety of visualization tools. Should have the filter system included Filter to start should only be by type, then include filtering by single name. 
- [ ] Scroll-able bar graph. Could either add the new inputs to the end or sort and display on every insert.