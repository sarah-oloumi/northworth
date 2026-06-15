const currencyFormatter = new Intl.NumberFormat("en-CA", {
  style: "currency",
  currency: "CAD",
  maximumFractionDigits: 0,
});

const sampleCsvText = `Date,Description,Amount,Account,Category
2026-06-01,Demo paycheque,5000.00,Demo chequing,Income
2026-06-02,Demo mortgage,-3200.50,Demo chequing,Housing
2026-06-03,Demo groceries,-124.80,Demo chequing,Groceries`;

const sampleOfxText = `<OFX>
<BANKTRANLIST>
<STMTTRN>
<DTPOSTED>20260601
<TRNAMT>5000.00
<NAME>Demo paycheque
</STMTTRN>
<STMTTRN>
<DTPOSTED>20260603
<TRNAMT>-64.25
<NAME>Demo fuel
</STMTTRN>
</BANKTRANLIST>
</OFX>`;

const sampleAccounts = [
  account("chequing", "Household chequing", "Chequing", "OnBudget", 124000),
  account("savings", "Emergency fund", "Savings", "OnBudget", 1800000),
  account("mortgage", "Demo mortgage", "Mortgage", "OffBudgetTracking", -42000000),
  account("investments", "Long-term portfolio", "RegisteredInvestment", "OffBudgetTracking", 18400000),
];

const sampleTransactions = [
  transaction(2026, 1, 1, "Salary deposit", 1480000, "chequing", "Income"),
  transaction(2026, 1, 3, "Mortgage payment", -475000, "chequing", "Housing"),
  transaction(2026, 1, 8, "Groceries", -118000, "chequing", "Groceries"),
  transaction(2026, 1, 12, "Fuel", -22000, "chequing", "Gas"),
  transaction(2026, 2, 1, "Salary deposit", 1480000, "chequing", "Income"),
  transaction(2026, 2, 4, "Mortgage payment", -475000, "chequing", "Housing"),
  transaction(2026, 2, 10, "Utilities", -54000, "chequing", "Bills"),
  transaction(2026, 2, 14, "Groceries", -123000, "chequing", "Groceries"),
  transaction(2026, 3, 1, "Salary deposit", 1480000, "chequing", "Income"),
  transaction(2026, 3, 4, "Mortgage payment", -475000, "chequing", "Housing"),
  transaction(2026, 3, 9, "Insurance", -31000, "chequing", "Insurance"),
  transaction(2026, 3, 15, "Portfolio transfer", -250000, "chequing", "Investments"),
  transaction(2026, 3, 20, "Mortgage balance update", -200000, "mortgage", "Debt"),
];

const dateRange = {
  start: { year: 2026, month: 1, day: 1 },
  end: { year: 2026, month: 3, day: 31 },
};

const budgetInput = {
  starting_to_budget: money(250000),
  starting_category_balances: [
    { category: "Groceries", amount: money(80000) },
    { category: "Gas", amount: money(25000) },
    { category: "Bills", amount: money(40000) },
  ],
  assignments: [
    { category: "Housing", amount: money(1450000) },
    { category: "Groceries", amount: money(375000) },
    { category: "Bills", amount: money(180000) },
    { category: "Gas", amount: money(90000) },
    { category: "Insurance", amount: money(90000) },
    { category: "Investments", amount: money(250000) },
  ],
  transactions: sampleTransactions,
  held_for_future_month: money(300000),
  rollover_overspending_categories: [],
};

const fallbackState = {
  mode: "Sample preview",
  cashFlow: {
    income: money(4440000),
    expenses: money(2023000),
    net_cash_flow: money(2417000),
    months: [
      flowMonth(2026, 1, 1480000, 615000),
      flowMonth(2026, 2, 1480000, 652000),
      flowMonth(2026, 3, 1480000, 756000),
    ],
  },
  spending: {
    total_spending: money(2023000),
    categories: [
      slice("Housing", 1425000, 3),
      slice("Investments", 250000, 1),
      slice("Groceries", 241000, 2),
      slice("Bills", 54000, 1),
      slice("Insurance", 31000, 1),
      slice("Gas", 22000, 1),
    ],
  },
  budget: {
    inflow: money(4440000),
    assigned: money(2435000),
    spent: money(2223000),
    held_for_future_month: money(300000),
    to_budget: money(1955000),
    overspending_deduction_next_month: money(0),
    categories: [
      budgetRow("Bills", 40000, 180000, -54000),
      budgetRow("Groceries", 80000, 375000, -241000),
      budgetRow("Gas", 25000, 90000, -22000),
      budgetRow("Housing", 0, 1450000, -1425000),
      budgetRow("Insurance", 0, 90000, -31000),
      budgetRow("Investments", 0, 250000, -250000),
    ],
  },
  netWorth: {
    assets: money(20324000),
    debts: money(42000000),
    net_worth: money(-21676000),
  },
  marketData: {
    policies: fallbackMarketDataPolicies(),
    freshness: {
      cadence: "Monthly",
      status: "Stale",
      age_days: 45,
      refresh_after_days: 31,
      refreshed_at: { year: 2026, month: 5, day: 1 },
      as_of: { year: 2026, month: 6, day: 15 },
    },
  },
};

document.addEventListener("DOMContentLoaded", () => {
  bindTabs();
  initializeCsvImport();
  initializeOfxImport();
  loadDashboard();
});

async function loadDashboard() {
  const state = await calculateState();
  renderState(state);
}

async function calculateState() {
  const invoke = window.__TAURI__?.core?.invoke;

  if (!invoke) {
    return fallbackState;
  }

  try {
    const input = {
      transactions: sampleTransactions,
      accounts: sampleAccounts,
      date_range: dateRange,
    };
    const [cashFlow, spending, budget, netWorth] = await Promise.all([
      invoke("build_cash_flow", { input }),
      invoke("build_spending_analysis", { input }),
      invoke("calculate_budget_month", { input: budgetInput }),
      invoke("build_net_worth", { accounts: sampleAccounts }),
    ]);
    const [marketPolicies, marketFreshness] = await Promise.all([
      invoke("list_market_data_refresh_policies"),
      invoke("assess_market_data_freshness", {
        input: {
          cadence: "Monthly",
          refreshed_at: { year: 2026, month: 5, day: 1 },
          as_of: { year: 2026, month: 6, day: 15 },
        },
      }),
    ]);

    return {
      mode: "Rust calculations",
      cashFlow,
      spending,
      budget,
      netWorth,
      marketData: {
        policies: marketPolicies,
        freshness: marketFreshness,
      },
    };
  } catch (error) {
    console.error(error);
    return {
      ...fallbackState,
      mode: "Sample preview",
    };
  }
}

function renderState(state) {
  text("run-state", state.mode);
  text("cash-flow-range", "Jan-Mar 2026");
  text("spending-total", formatMoney(state.spending.total_spending));
  text("cash-flow-net", formatMoney(state.cashFlow.net_cash_flow));
  text("budget-to-budget", formatMoney(state.budget.to_budget));

  renderMetrics(state);
  renderCashChart(state.cashFlow.months);
  renderSpendingMix(state.spending.categories, state.spending.total_spending.cents);
  renderCashFlowTable(state.cashFlow.months);
  renderBudgetTable(state.budget.categories);
  renderMarketDataSettings(state.marketData);
}

function renderMetrics(state) {
  const metrics = [
    ["Income", state.cashFlow.income],
    ["Expenses", state.cashFlow.expenses],
    ["Net cash flow", state.cashFlow.net_cash_flow],
    ["Net worth", state.netWorth.net_worth],
  ];
  const container = clear("dashboard-metrics");

  metrics.forEach(([label, value]) => {
    container.append(
      element("div", "metric", [
        element("span", null, label),
        element("strong", null, formatMoney(value)),
      ]),
    );
  });
}

function renderCashChart(months) {
  const maxAmount = Math.max(
    ...months.flatMap((month) => [month.income.cents, month.expenses.cents]),
    1,
  );
  const container = clear("cash-chart");

  months.forEach((month) => {
    const incomeHeight = percentClass(month.income.cents, maxAmount);
    const expenseHeight = percentClass(month.expenses.cents, maxAmount);
    const bars = element("div", "bars", [
      element("span", `bar income h-${incomeHeight}`),
      element("span", `bar expense h-${expenseHeight}`),
    ]);

    bars.setAttribute("aria-label", formatMonth(month.month));
    container.append(
      element("div", "chart-month", [
        bars,
        element("strong", null, formatMonth(month.month)),
        element("span", null, formatMoney(month.net_cash_flow)),
      ]),
    );
  });
}

function renderSpendingMix(categories, totalCents) {
  const container = clear("spending-mix");

  categories.forEach((category) => {
    const percent = totalCents > 0 ? Math.round((category.amount.cents / totalCents) * 100) : 0;
    const trackFill = element("span", `w-${roundToStep(percent)}`);

    container.append(
      element("div", "slice", [
        element("div", null, [
          element("strong", null, formatCategory(category.category)),
          element("span", null, `${category.transaction_count} transactions`),
        ]),
        element("div", "slice-amount", [
          element("strong", null, formatMoney(category.amount)),
          element("span", null, `${percent}%`),
        ]),
        element("div", "slice-track", [trackFill]),
      ]),
    );
  });
}

function renderCashFlowTable(months) {
  const container = clear("cash-flow-table");

  container.append(tableRow(["Month", "Income", "Expenses", "Net"], "table-row table-head"));
  months.forEach((month) => {
    container.append(
      tableRow([
        formatMonth(month.month),
        formatMoney(month.income),
        formatMoney(month.expenses),
        formatMoney(month.net_cash_flow),
      ]),
    );
  });
}

function renderBudgetTable(categories) {
  const container = clear("budget-table");

  container.append(tableRow(["Category", "Assigned", "Activity", "Available"], "table-row budget-head"));
  categories.forEach((category) => {
    container.append(
      tableRow([
        formatCategory(category.category),
        formatMoney(category.assigned),
        formatMoney(category.activity),
        formatMoney(category.available),
      ]),
    );
  });
}

function renderMarketDataSettings(marketData) {
  text("market-freshness-status", formatFreshness(marketData.freshness));

  const container = clear("market-cadence-list");

  marketData.policies.forEach((policy) => {
    const isSelected = policy.cadence === marketData.freshness.cadence;
    const row = element("div", isSelected ? "cadence-row selected" : "cadence-row", [
      element("div", null, [
        element("strong", null, policy.label),
        element("span", null, policy.description),
      ]),
      element("span", "cadence-days", `${policy.refresh_after_days} days`),
    ]);

    if (isSelected) {
      row.setAttribute("aria-current", "true");
    }

    container.append(row);
  });
}

function initializeCsvImport() {
  const csvText = document.getElementById("csv-text");
  const csvFile = document.getElementById("csv-file");
  const amountMode = document.getElementById("csv-amount-mode");
  const previewButton = document.getElementById("csv-preview-button");

  csvText.value = sampleCsvText;
  updateCsvHeaderControls();
  updateAmountModeControls();

  csvText.addEventListener("input", updateCsvHeaderControls);
  csvFile.addEventListener("change", readSelectedCsvFile);
  amountMode.addEventListener("change", updateAmountModeControls);
  previewButton.addEventListener("click", previewCsvImport);

  document.querySelectorAll(".mapping-grid select").forEach((select) => {
    select.addEventListener("change", previewCsvImport);
  });

  previewCsvImport();
}

function initializeOfxImport() {
  const ofxText = document.getElementById("ofx-text");
  const ofxFile = document.getElementById("ofx-file");
  const previewButton = document.getElementById("ofx-preview-button");

  ofxText.value = sampleOfxText;
  ofxText.addEventListener("input", previewOfxImport);
  ofxFile.addEventListener("change", readSelectedOfxFile);
  previewButton.addEventListener("click", previewOfxImport);

  document.querySelectorAll("[data-import-mode]").forEach((button) => {
    button.addEventListener("click", () => showImportMode(button.dataset.importMode));
  });

  previewOfxImport();
}

function showImportMode(mode) {
  document.querySelectorAll("[data-import-mode]").forEach((button) => {
    button.classList.toggle("active", button.dataset.importMode === mode);
  });
  document.querySelectorAll("[data-import-panel]").forEach((panel) => {
    panel.classList.toggle("active", panel.dataset.importPanel === mode);
  });
  document.querySelectorAll("[data-import-preview]").forEach((panel) => {
    panel.classList.toggle("active", panel.dataset.importPreview === mode);
  });
}

function readSelectedCsvFile(event) {
  const file = event.target.files?.[0];

  if (!file) {
    return;
  }

  const reader = new FileReader();

  reader.addEventListener("load", () => {
    document.getElementById("csv-text").value = String(reader.result ?? "");
    updateCsvHeaderControls();
    previewCsvImport();
  });
  reader.readAsText(file);
}

function readSelectedOfxFile(event) {
  const file = event.target.files?.[0];

  if (!file) {
    return;
  }

  const reader = new FileReader();

  reader.addEventListener("load", () => {
    document.getElementById("ofx-text").value = String(reader.result ?? "");
    previewOfxImport();
  });
  reader.readAsText(file);
}

function updateCsvHeaderControls() {
  const csvText = document.getElementById("csv-text").value;
  const rows = parseCsvRows(csvText);
  const headers = rows[0] ?? [];
  const inferredDateColumn = inferColumn(headers, ["date", "posted date"]);

  populateColumnSelect("csv-date-column", headers, inferredDateColumn);
  populateColumnSelect(
    "csv-description-column",
    headers,
    inferColumn(headers, ["description", "memo", "name", "payee"]),
  );
  populateColumnSelect("csv-amount-column", headers, inferColumn(headers, ["amount", "transaction amount"]));
  populateColumnSelect("csv-debit-column", headers, inferColumn(headers, ["debit", "withdrawal"]));
  populateColumnSelect("csv-credit-column", headers, inferColumn(headers, ["credit", "deposit"]));
  populateColumnSelect("csv-account-column", headers, inferColumn(headers, ["account"], true), true);
  populateColumnSelect("csv-category-column", headers, inferColumn(headers, ["category"], true), true);
  document.getElementById("csv-date-format").value = inferDateFormat(rows, headers, inferredDateColumn);
}

function populateColumnSelect(id, headers, selectedValue, allowNone = false) {
  const select = document.getElementById(id);
  const options = [];

  if (allowNone) {
    options.push(option("", "None"));
  }

  headers.forEach((header) => options.push(option(header, header)));
  select.replaceChildren(...options);
  select.value = selectedValue ?? "";
}

function updateAmountModeControls() {
  const isSplit = document.getElementById("csv-amount-mode").value === "split";

  document.querySelectorAll(".amount-single").forEach((node) => {
    node.classList.toggle("hidden", isSplit);
  });
  document.querySelectorAll(".amount-split").forEach((node) => {
    node.classList.toggle("hidden", !isSplit);
  });
}

async function previewCsvImport() {
  const csvText = document.getElementById("csv-text").value;
  const mapping = buildCsvMapping();
  const invoke = window.__TAURI__?.core?.invoke;
  let preview;

  if (invoke) {
    try {
      preview = await invoke("preview_csv_transactions", { csvText, mapping });
    } catch (error) {
      console.error(error);
      preview = previewCsvImportFallback(csvText, mapping);
    }
  } else {
    preview = previewCsvImportFallback(csvText, mapping);
  }

  renderCsvPreview(preview);
}

async function previewOfxImport() {
  const ofxText = document.getElementById("ofx-text").value;
  const invoke = window.__TAURI__?.core?.invoke;
  let preview;

  if (invoke) {
    try {
      preview = await invoke("preview_ofx_transactions", { ofxText });
    } catch (error) {
      console.error(error);
      preview = previewOfxImportFallback(ofxText);
    }
  } else {
    preview = previewOfxImportFallback(ofxText);
  }

  renderOfxPreview(preview);
}

function buildCsvMapping() {
  const amountMode = document.getElementById("csv-amount-mode").value;
  const amountMapping =
    amountMode === "split"
      ? {
          Split: {
            debit_column: value("csv-debit-column"),
            credit_column: value("csv-credit-column"),
          },
        }
      : {
          Single: {
            column: value("csv-amount-column"),
            convention: value("csv-sign-convention"),
          },
        };

  return {
    date_column: value("csv-date-column"),
    date_format: value("csv-date-format"),
    description_column: value("csv-description-column"),
    amount_mapping: amountMapping,
    account_column: value("csv-account-column") || null,
    category_column: value("csv-category-column") || null,
  };
}

function previewCsvImportFallback(csvText, mapping) {
  const rows = parseCsvRows(csvText);
  const headers = rows[0] ?? [];
  const transactions = [];
  const errors = [];

  rows.slice(1).forEach((row, index) => {
    const sourceRow = index + 2;
    const transaction = fallbackCsvTransaction(row, headers, sourceRow, mapping, errors);

    if (transaction) {
      transactions.push(transaction);
    }
  });

  return { transactions, errors };
}

function fallbackCsvTransaction(row, headers, sourceRow, mapping, errors) {
  const dateValue = csvValue(row, headers, mapping.date_column);
  const description = csvValue(row, headers, mapping.description_column);
  const amount = fallbackCsvAmount(row, headers, sourceRow, mapping.amount_mapping, errors);
  const date = parseFallbackDate(dateValue, mapping.date_format);

  if (!date) {
    errors.push(csvError(sourceRow, mapping.date_column, `Invalid date \`${dateValue}\``));
  }

  if (!description) {
    errors.push(csvError(sourceRow, mapping.description_column, "Missing description"));
  }

  if (!date || !description || !amount) {
    return null;
  }

  return {
    source_row: sourceRow,
    date,
    description,
    amount,
    account_id: null,
    account_name: mapping.account_column ? csvValue(row, headers, mapping.account_column) || null : null,
    category: mapping.category_column ? csvValue(row, headers, mapping.category_column) || null : null,
    import_status: "PendingReview",
  };
}

function fallbackCsvAmount(row, headers, sourceRow, amountMapping, errors) {
  if (amountMapping.Single) {
    const raw = csvValue(row, headers, amountMapping.Single.column);
    const cents = parseMoneyCents(raw);

    if (cents === null) {
      errors.push(csvError(sourceRow, amountMapping.Single.column, `Invalid amount \`${raw}\``));
      return null;
    }

    return money(amountMapping.Single.convention === "PositiveIsOutflow" ? -cents : cents);
  }

  const debit = parseMoneyCents(csvValue(row, headers, amountMapping.Split.debit_column) || "0");
  const credit = parseMoneyCents(csvValue(row, headers, amountMapping.Split.credit_column) || "0");

  if (debit === null || credit === null || (debit === 0 && credit === 0)) {
    errors.push(csvError(sourceRow, null, "Expected a debit or credit amount"));
    return null;
  }

  return money(credit - debit);
}

function renderCsvPreview(preview) {
  text("csv-preview-count", `${preview.transactions.length} rows`);
  text("csv-preview-status", preview.errors.length ? `${preview.errors.length} errors` : "preview only");

  const errors = clear("csv-errors");
  preview.errors.forEach((error) => {
    const location = error.row ? `Row ${error.row}` : "Mapping";
    errors.append(element("div", "import-error", `${location}: ${error.message}`));
  });

  const table = clear("csv-preview-table");
  table.append(tableRow(["Row", "Date", "Description", "Amount"], "table-row table-head"));
  preview.transactions.slice(0, 8).forEach((transaction) => {
    table.append(
      tableRow([
        String(transaction.source_row),
        formatDate(transaction.date),
        transaction.description,
        formatMoney(transaction.amount),
      ]),
    );
  });
}

function previewOfxImportFallback(ofxText) {
  const blocks = taggedBlocks(ofxText, "STMTTRN");

  if (!blocks.length) {
    return {
      transactions: [],
      errors: [{ transaction_index: null, field: null, message: "No OFX/QFX transactions found" }],
    };
  }

  const transactions = [];
  const errors = [];

  blocks.forEach((block, index) => {
    const sourceRow = index + 1;
    const date = parseOfxDate(tagValue(block, "DTPOSTED"));
    const amount = parseMoneyCents(tagValue(block, "TRNAMT") ?? "");
    const description = tagValue(block, "NAME") || tagValue(block, "MEMO");

    if (!date) {
      errors.push(ofxError(sourceRow, "DTPOSTED", "Missing or invalid posted date"));
    }

    if (amount === null) {
      errors.push(ofxError(sourceRow, "TRNAMT", "Missing or invalid transaction amount"));
    }

    if (!description) {
      errors.push(ofxError(sourceRow, "NAME", "Missing transaction description"));
    }

    if (date && amount !== null && description) {
      transactions.push({
        source_row: sourceRow,
        date,
        description,
        amount: money(amount),
        account_id: null,
        account_name: null,
        category: null,
        import_status: "PendingReview",
      });
    }
  });

  return { transactions, errors };
}

function renderOfxPreview(preview) {
  text("ofx-preview-count", `${preview.transactions.length} rows`);
  text("ofx-preview-status", preview.errors.length ? `${preview.errors.length} errors` : "preview only");

  const errors = clear("ofx-errors");
  preview.errors.forEach((error) => {
    const location = error.transaction_index ? `Transaction ${error.transaction_index}` : "Statement";
    errors.append(element("div", "import-error", `${location}: ${error.message}`));
  });

  const table = clear("ofx-preview-table");
  table.append(tableRow(["Txn", "Date", "Description", "Amount"], "table-row table-head"));
  preview.transactions.slice(0, 8).forEach((transaction) => {
    table.append(
      tableRow([
        String(transaction.source_row),
        formatDate(transaction.date),
        transaction.description,
        formatMoney(transaction.amount),
      ]),
    );
  });
}

function bindTabs() {
  document.querySelectorAll("[data-view]").forEach((tab) => {
    tab.addEventListener("click", (event) => {
      event.preventDefault();
      showView(tab.dataset.view);
      history.replaceState(null, "", `#${tab.dataset.view}`);
    });
  });

  window.addEventListener("hashchange", () => {
    showView(window.location.hash.replace("#", "") || "dashboard");
  });

  showView(window.location.hash.replace("#", "") || "dashboard");
}

function showView(view) {
  const tab =
    document.querySelector(`[data-view="${view}"]`) ?? document.querySelector('[data-view="dashboard"]');

  if (!tab) {
    return;
  }

  document.querySelectorAll("[data-view]").forEach((item) => {
    item.classList.toggle("active", item === tab);
  });
  document.querySelectorAll("[data-panel]").forEach((panel) => {
    panel.classList.toggle("active", panel.dataset.panel === tab.dataset.view);
  });

  text("view-title", tab.querySelector(".tab-label").textContent);
}

function account(id, name, kind, budgetTreatment, balanceCents) {
  return {
    id,
    name,
    kind,
    budget_treatment: budgetTreatment,
    status: "Open",
    current_balance: money(balanceCents),
  };
}

function transaction(year, month, day, description, cents, accountId, category) {
  return {
    source_row: 2,
    date: { year, month, day },
    description,
    amount: money(cents),
    account_id: accountId,
    account_name: accountId,
    category,
    import_status: "Accepted",
  };
}

function flowMonth(year, month, incomeCents, expenseCents) {
  return {
    month: { year, month },
    income: money(incomeCents),
    expenses: money(expenseCents),
    net_cash_flow: money(incomeCents - expenseCents),
  };
}

function slice(category, cents, transactionCount) {
  return {
    category,
    amount: money(cents),
    transaction_count: transactionCount,
  };
}

function budgetRow(category, startingCents, assignedCents, activityCents) {
  const availableCents = startingCents + assignedCents + activityCents;

  return {
    category,
    starting_balance: money(startingCents),
    assigned: money(assignedCents),
    activity: money(activityCents),
    available: money(availableCents),
    rollover_to_next_month: money(Math.max(availableCents, 0)),
    overspent: money(Math.min(availableCents, 0) * -1),
  };
}

function money(cents) {
  return {
    cents,
    currency: "Cad",
  };
}

function formatMoney(value) {
  return currencyFormatter.format(value.cents / 100);
}

function formatCategory(category) {
  if (typeof category === "string") {
    return category.replace(/([a-z])([A-Z])/g, "$1 $2");
  }

  return category.Other ?? "Other";
}

function formatMonth(month) {
  const date = new Date(month.year, month.month - 1, 1);
  return date.toLocaleDateString("en-CA", { month: "short" });
}

function text(id, value) {
  document.getElementById(id).textContent = value;
}

function value(id) {
  return document.getElementById(id).value;
}

function clear(id) {
  const node = document.getElementById(id);
  node.replaceChildren();
  return node;
}

function element(tagName, className, childrenOrText) {
  const node = document.createElement(tagName);

  if (className) {
    node.className = className;
  }

  if (typeof childrenOrText === "string") {
    node.textContent = childrenOrText;
  } else if (Array.isArray(childrenOrText)) {
    node.append(...childrenOrText);
  }

  return node;
}

function option(value, label) {
  const node = document.createElement("option");

  node.value = value;
  node.textContent = label;

  return node;
}

function tableRow(cells, className = "table-row") {
  const row = element("div", className);

  cells.forEach((cell, index) => {
    row.append(element(index === 0 && className === "table-row" ? "strong" : "span", null, cell));
  });

  return row;
}

function fallbackMarketDataPolicies() {
  return [
    marketPolicy("Monthly", "Monthly", 31, "Refresh prices and issuer facts when a cache is more than 31 days old."),
    marketPolicy("Quarterly", "Quarterly", 92, "Refresh when a cache is more than 92 days old."),
    marketPolicy("Biannual", "Biannual", 183, "Refresh when a cache is more than 183 days old."),
    marketPolicy("Yearly", "Yearly", 366, "Refresh when a cache is more than 366 days old."),
  ];
}

function marketPolicy(cadence, label, refreshAfterDays, description) {
  return {
    cadence,
    label,
    refresh_after_days: refreshAfterDays,
    description,
  };
}

function formatFreshness(freshness) {
  if (freshness.status === "FutureDated") {
    return "future dated";
  }

  if (freshness.status === "Stale") {
    return `stale by ${freshness.age_days - freshness.refresh_after_days} days`;
  }

  return `${freshness.age_days} days old`;
}

function formatDate(date) {
  return `${date.year}-${String(date.month).padStart(2, "0")}-${String(date.day).padStart(2, "0")}`;
}

function inferColumn(headers, candidates, allowBlank = false) {
  const normalized = new Map(headers.map((header) => [header.trim().toLowerCase(), header]));

  for (const candidate of candidates) {
    if (normalized.has(candidate)) {
      return normalized.get(candidate);
    }
  }

  return allowBlank ? "" : headers[0] ?? "";
}

function inferDateFormat(rows, headers, dateColumn) {
  const sample = csvValue(rows[1] ?? [], headers, dateColumn);
  const parts = sample.replaceAll("/", "-").split("-").map(Number);

  if (parts.length !== 3 || parts.some((part) => Number.isNaN(part))) {
    return "IsoYearMonthDay";
  }

  if (parts[0] > 31) {
    return "IsoYearMonthDay";
  }

  if (parts[0] > 12) {
    return "DayMonthYear";
  }

  return "MonthDayYear";
}

function csvValue(row, headers, column) {
  const index = headers.findIndex((header) => header.trim().toLowerCase() === column.trim().toLowerCase());

  return index >= 0 ? row[index]?.trim() ?? "" : "";
}

function csvError(row, column, message) {
  return { row, column, message };
}

function ofxError(transactionIndex, field, message) {
  return { transaction_index: transactionIndex, field, message };
}

function parseFallbackDate(value, format) {
  const parts = value.trim().replaceAll("/", "-").split("-").map(Number);

  if (parts.length !== 3 || parts.some((part) => Number.isNaN(part))) {
    return null;
  }

  if (format === "MonthDayYear") {
    return { year: parts[2], month: parts[0], day: parts[1] };
  }

  if (format === "DayMonthYear") {
    return { year: parts[2], month: parts[1], day: parts[0] };
  }

  return { year: parts[0], month: parts[1], day: parts[2] };
}

function parseMoneyCents(value) {
  const trimmed = value.trim();

  if (!trimmed) {
    return 0;
  }

  const parenthesized = trimmed.startsWith("(") && trimmed.endsWith(")");
  const cleaned = trimmed.replace(/[,$]/g, "").replace(/cad/gi, "").replace(/[()]/g, "").trim();
  const parsed = Number(cleaned);

  if (!Number.isFinite(parsed)) {
    return null;
  }

  return Math.round(Math.abs(parsed) * 100) * (parenthesized || parsed < 0 ? -1 : 1);
}

function parseOfxDate(value) {
  if (!value) {
    return null;
  }

  const digits = value.trim().match(/^\d+/)?.[0] ?? "";

  if (digits.length < 8) {
    return null;
  }

  return {
    year: Number(digits.slice(0, 4)),
    month: Number(digits.slice(4, 6)),
    day: Number(digits.slice(6, 8)),
  };
}

function taggedBlocks(contents, tag) {
  const blocks = [];
  const upper = contents.toUpperCase();
  const openTag = `<${tag}>`;
  const closeTag = `</${tag}>`;
  let searchFrom = 0;

  while (true) {
    const openIndex = upper.indexOf(openTag, searchFrom);

    if (openIndex < 0) {
      break;
    }

    const blockStart = openIndex + openTag.length;
    const closeIndex = upper.indexOf(closeTag, blockStart);

    if (closeIndex < 0) {
      break;
    }

    blocks.push(contents.slice(blockStart, closeIndex));
    searchFrom = closeIndex + closeTag.length;
  }

  return blocks;
}

function tagValue(contents, tag) {
  const upper = contents.toUpperCase();
  const openTag = `<${tag}>`;
  const start = upper.indexOf(openTag);

  if (start < 0) {
    return null;
  }

  const valueStart = start + openTag.length;
  const rest = contents.slice(valueStart);
  const valueEnd = rest.indexOf("<");
  const value = rest.slice(0, valueEnd < 0 ? rest.length : valueEnd).trim();

  return value || null;
}

function parseCsvRows(csvText) {
  const rows = [];
  let row = [];
  let value = "";
  let quoted = false;

  for (let index = 0; index < csvText.length; index += 1) {
    const char = csvText[index];
    const next = csvText[index + 1];

    if (char === '"' && quoted && next === '"') {
      value += '"';
      index += 1;
    } else if (char === '"') {
      quoted = !quoted;
    } else if (char === "," && !quoted) {
      row.push(value.trim());
      value = "";
    } else if ((char === "\n" || char === "\r") && !quoted) {
      if (char === "\r" && next === "\n") {
        index += 1;
      }
      row.push(value.trim());
      rows.push(row);
      row = [];
      value = "";
    } else {
      value += char;
    }
  }

  if (value || row.length) {
    row.push(value.trim());
    rows.push(row);
  }

  return rows.filter((parsedRow) => parsedRow.some(Boolean));
}

function percentClass(value, max) {
  return Math.max(10, roundToStep((value / max) * 100));
}

function roundToStep(value) {
  return Math.min(100, Math.max(0, Math.round(value / 5) * 5));
}
