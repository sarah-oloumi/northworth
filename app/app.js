const currencyFormatter = new Intl.NumberFormat("en-CA", {
  style: "currency",
  currency: "CAD",
  maximumFractionDigits: 0,
});

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

function percentClass(value, max) {
  return Math.max(10, roundToStep((value / max) * 100));
}

function roundToStep(value) {
  return Math.min(100, Math.max(0, Math.round(value / 5) * 5));
}
