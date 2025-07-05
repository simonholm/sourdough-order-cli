# Next Steps for Sourdough Order CLI

## Project Status Summary

The sourdough order CLI is a functional Rust application with solid foundations for micro-bakery order management. Core features like interactive order creation, SMS integration, and JSON persistence are working well. The project is ready for the next phase of development.

## Immediate Next Steps (High Priority)

### 1. Integrate Scratchpad Modules
**Status:** Ready to implement  
**Files:** `scratchpad/menu.rs`, `scratchpad/order.rs`  
**Goal:** Move well-structured modules into main application

- [ ] Integrate `menu.rs` module to replace hardcoded menu items in `main.rs`
- [ ] Integrate `order.rs` module with improved order structure
- [ ] Remove duplicate code between main and scratchpad
- [ ] Update `Cargo.toml` if needed for new module structure

### 2. Implement Proper CLI Parsing
**Status:** Critical for user experience  
**Current:** String matching in `main.rs`  
**Goal:** Professional CLI interface with `clap`

- [ ] Add `clap` dependency to `Cargo.toml`
- [ ] Replace current argument parsing with `clap` framework
- [ ] Add help documentation for all commands
- [ ] Implement subcommands: `new`, `list-today`, `list-all`, `demo`, `reset`
- [ ] Add command-line options and flags

### 3. Add Order Status Management
**Status:** Planned feature from requirements  
**Goal:** Track order lifecycle (pending/confirmed/completed)

- [ ] Add status field to order structure
- [ ] Implement status update commands
- [ ] Add status filtering to list commands
- [ ] Update JSON schema to include status

### 4. Improve Date Filtering
**Status:** Mentioned in requirements  
**Goal:** Better date-based order management

- [ ] Add "next N days" functionality
- [ ] Implement date range filtering
- [ ] Add pickup date validation
- [ ] Create date-based reporting commands

## Medium Priority

### 5. Add Unit Tests
**Status:** Missing test coverage  
**Goal:** Ensure code reliability

- [ ] Set up Rust test framework
- [ ] Add unit tests for core order functions
- [ ] Add integration tests for CLI commands
- [ ] Test SMS integration with mocking
- [ ] Add test data fixtures

### 6. Migrate to SQLite
**Status:** Planned architecture improvement  
**Goal:** Better data persistence and querying

- [ ] Add `rusqlite` dependency
- [ ] Design database schema
- [ ] Implement migration from JSON to SQLite
- [ ] Add database initialization and setup
- [ ] Update all CRUD operations to use database

### 7. Start Web Interface
**Status:** Planned feature  
**Goal:** Mobile-friendly web interface for Instagram integration

- [ ] Add `axum` and `askama` dependencies
- [ ] Create basic web server structure
- [ ] Design mobile-friendly HTML templates
- [ ] Implement order viewing and creation via web
- [ ] Add responsive CSS for mobile devices

## Low Priority

### 8. Enhanced Features

- [ ] Export functionality (printable order lists)
- [ ] Customer management (repeat customer tracking)
- [ ] Reporting and analytics
- [ ] Order modification and cancellation
- [ ] Bulk operations for orders

### 9. Production Readiness

- [ ] Error handling improvements
- [ ] Logging and monitoring
- [ ] Configuration management
- [ ] Security review
- [ ] Performance optimization

## Technical Debt

### Current Limitations to Address
- No proper CLI argument parsing
- Hardcoded menu items in `main.rs`
- Duplicate order structures between main and scratchpad
- Manual date validation
- Limited error handling

### Code Quality Improvements
- [ ] Add comprehensive error handling
- [ ] Improve code documentation
- [ ] Implement proper logging
- [ ] Add input validation
- [ ] Standardize error messages

## Architecture Goals

### Short Term (1-2 weeks)
- Modular code structure with integrated scratchpad modules
- Professional CLI interface with `clap`
- Order status management system

### Medium Term (1-2 months)
- Database persistence with SQLite
- Comprehensive test coverage
- Basic web interface prototype

### Long Term (3-6 months)
- Production-ready web interface
- Mobile-responsive design
- Instagram integration capabilities
- Advanced reporting features

## Success Metrics

- [ ] All scratchpad modules integrated
- [ ] CLI commands work with `clap` framework
- [ ] Order status tracking functional
- [ ] Test coverage > 80%
- [ ] Web interface serves basic pages
- [ ] Database migration complete

## Getting Started

To begin implementing these improvements:

1. **Start with Module Integration** - This will clean up the codebase and make future changes easier
2. **Add CLI Framework** - This improves user experience immediately
3. **Implement Tests** - This ensures stability as you add new features
4. **Consider Database Migration** - This prepares for web interface development

The project has excellent foundations and is ready for these enhancements. Focus on one high-priority item at a time to maintain momentum and ensure quality.