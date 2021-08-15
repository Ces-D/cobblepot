# Cobblepot

<p align="center">
<img src="https://media.giphy.com/media/UHZMvURcKk8IU/giphy.gif" ></img>
</p>

This is a command-line finance tool meant primarily for personal use. The syntax of the command inputs is influenced heavily by [beancount](https://github.com/beancount/beancount), a more robust finance CLI tool.

## Purpose

This is a great tool if you are not interested in all the bells and whistles of complicated CLI finance tools. There is little syntax to learn and your input format is an open road. Cobblepot has simple syntax with detailed help text from the command line. You can generate reports and summaries at any moment without becoming confused by the variety.

## Definitions

- **Assets**. (+) Asset accounts represent something the owner has. A canonical example is banking accounts. Another one is a “cash” account, which counts how much money is in your wallet. Investments are also assets (their units aren’t dollars in this case, but rather some number of shares of some mutual fund or stock). Finally, if you own a home, the home itself is considered an asset (and its market value fluctuates over time).

- **Liabilities**. (-) A liability account represents something the owner owes. The most common example is a credit card. Again, the statement provided by your bank will show positive numbers, but from your own perspective, they are negative numbers. A loan is also a liability account. For example, if you take out a mortgage on a home, this is money you owe, and will be tracked by an account with a negative amount. As you pay off the mortgage every month the negative number goes up, that is, its absolute value gets smaller and smaller over time (e.g., -120,000 -> -117,345).

- **Expenses**. (+) An expense account represents something you’ve received, perhaps by exchanging something else to purchase it. This type of account will seem pretty natural: food, drinks, clothing, rent, flights, hotels and most other categories of things you typically spend your disposable income on. However, taxes are also typically tracked by an expense account: when you receive some salary income, the amount of taxes withheld at the source is recorded immediately as an expense. Think of it as paying for government services you receive throughout the year.

- **Income**. (-) An income account is used to count something you’ve given away in order to receive something else (typically assets or expenses). For most people with jobs, that is the value of their time (a salary income). Specifically, here we’re talking about the gross income. For example, if you’re earning a salary of $120,000/year, that number is $120,000, not whatever amount remains after paying for taxes. Other types of income includes dividends received from investments, or interest paid from bonds held. There are also a number of oddball things received you might record as income, such the value of rewards received, e.g., cash back from a credit card, or monetary gifts from someone.
- **Equity**. ()And summing up all the Equity accounts clearly tells us what’s our stake in the entity, in other words, if you used the assets to pay off all the liabilities, how much is left in the business… how much it’s worth.

  - Previous Earnings or Retained Earnings. An account used to hold the sum total of Income & Expenses balances from the beginning of time until the beginning of a reporting period. This is the account we were referring to in the previous section.

  - Current Earnings, also called Net Income. An account used to contain the sum of Income & Expenses balances incurred during the reporting period. They are filled in by “clearing” the Income & Expenses accounts at the end of the reporting period.

  - Opening Balances. An equity account used to counterbalance deposits used to initialize accounts. This type of account is used when we truncate the past history of transactions, but we also need to ensure that an account’s balance begins its history with a particular amount.

### Details Format

`<type> : <country> : <institution> : <account>`
There are many ways ot abuse the input format for details. This is the method that I will be following.
